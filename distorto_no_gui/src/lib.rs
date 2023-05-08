use nih_plug::prelude::*;
use std::sync::Arc;

mod filter;

/// The length of silence after which the signal should start fading out into silence. This is to
/// avoid outputting a constant DC signal.
const SILENCE_FADEOUT_START_MS: f32 = 1000.0;
/// The time it takes after `SILENCE_FADEOUT_START_MS` to fade from a full scale DC signal to silence.
const SILENCE_FADEOUT_END_MS: f32 = SILENCE_FADEOUT_START_MS + 1000.0;

/// The center frequency for our optional bandpass filter, in Hertz.
// const BP_FREQUENCY: f32 = 5500.0;

struct Distorto {
    params: Arc<DistortoParams>,

    sample_rate: f32,
    bp_filters: Vec<[filter::Biquad<f32>; 4]>,
    num_silent_samples: u32,
    silence_fadeout_start_samples: u32,
    silence_fadeout_end_samples: u32,
    silence_fadeout_length_samples: u32,
}

#[derive(Params)]
struct DistortoParams {
    #[id = "output"]
    output_gain: FloatParam,

    #[id = "distortion"]
    distortion: FloatParam,
}

impl Default for Distorto {
    fn default() -> Self {
        Self {
            params: Arc::new(DistortoParams::default()),

            sample_rate: 1.0,
            bp_filters: Vec::new(),

            num_silent_samples: 0,
            silence_fadeout_start_samples: 0,
            silence_fadeout_end_samples: 0,
            silence_fadeout_length_samples: 0,
        }
    }
}

impl Default for DistortoParams {
    fn default() -> Self {
        Self {
            output_gain: FloatParam::new(
                "Output Gain",
                util::db_to_gain(-24.0),
                // Because we're representing gain as decibels the range is already logarithmic
                FloatRange::Linear {
                    min: util::db_to_gain(-24.0),
                    max: util::db_to_gain(0.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(10.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            distortion: FloatParam::new(
                "Distortion",
                0.0,
                // This ramps up hard, so we'll make sure the 'usable' (for a lack of a better word)
                // value range is larger
                FloatRange::Skewed {
                    min: 0.0,
                    max: 1.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(30.0))
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),
        }
    }
}

impl Plugin for Distorto {
    const NAME: &'static str = "Distorto";
    const VENDOR: &'static str = "Hector";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "contact@hectorbennett.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;

        let num_output_channels = audio_io_layout
            .main_output_channels
            .expect("Plugin does not have a main output")
            .get() as usize;
        self.bp_filters
            .resize(num_output_channels, [filter::Biquad::default(); 4]);

        self.silence_fadeout_start_samples =
            (SILENCE_FADEOUT_START_MS / 1000.0 * buffer_config.sample_rate).round() as u32;
        self.silence_fadeout_end_samples =
            (SILENCE_FADEOUT_END_MS / 1000.0 * buffer_config.sample_rate).round() as u32;
        self.silence_fadeout_length_samples =
            self.silence_fadeout_end_samples - self.silence_fadeout_start_samples;

        true
    }

    fn reset(&mut self) {
        for filters in &mut self.bp_filters {
            for filter in filters {
                filter.reset();
            }
        }

        // Start with silence, so we don't immediately output a DC signal if the plugin is inserted
        // on a silent channel
        self.num_silent_samples = self.silence_fadeout_end_samples;
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for mut channel_samples in buffer.iter_samples() {
            let output_gain = self.params.output_gain.smoothed.next();

            let apply_bp_filters = self.params.distortion.smoothed.previous_value() > 0.0;

            let mut is_silent = true;
            for (sample, bp_filters) in channel_samples.iter_mut().zip(&mut self.bp_filters) {
                is_silent &= *sample == 0.0;

                // For better performance we can move this conditional to an outer loop, but right
                // now it shouldn't be too bad
                if apply_bp_filters {
                    for filter in bp_filters {
                        *sample = filter.process(*sample);
                    }
                }

                *sample = if *sample >= 0.0 { 1.0 } else { -1.0 } * output_gain;
            }

            // To avoid outputting a constant DC signal even when there's no input we'll slowly fade
            // into silence
            if is_silent {
                self.num_silent_samples += 1;

                if self.num_silent_samples >= self.silence_fadeout_end_samples {
                    for sample in channel_samples {
                        *sample = 0.0;
                    }
                } else if self.num_silent_samples >= self.silence_fadeout_start_samples {
                    let fadeout_gain = 1.0
                        - ((self.num_silent_samples - self.silence_fadeout_start_samples) as f32
                            / self.silence_fadeout_length_samples as f32);

                    for sample in channel_samples {
                        *sample *= fadeout_gain;
                    }
                }
            } else {
                self.num_silent_samples = 0;
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for Distorto {
    const CLAP_ID: &'static str = "hectorbennett.distorto";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Distortoooo");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Limiter,
        ClapFeature::Distortion,
    ];
}

impl Vst3Plugin for Distorto {
    const VST3_CLASS_ID: [u8; 16] = *b"Distortooooooooo";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Distortion];
}

nih_export_clap!(Distorto);
nih_export_vst3!(Distorto);
