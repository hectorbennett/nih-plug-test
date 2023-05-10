use nih_plug::prelude::*;
use std::sync::Arc;

mod drive;

struct UsingWebview {
    params: Arc<UsingWebviewParams>,

    sample_rate: f32,
}

#[derive(Params)]
struct UsingWebviewParams {
    #[id = "gain"]
    output_gain: FloatParam,

    #[id = "drive"]
    drive: FloatParam,
}

impl Default for UsingWebview {
    fn default() -> Self {
        Self {
            params: Arc::new(UsingWebviewParams::default()),
            sample_rate: 1.0,
        }
    }
}

impl Default for UsingWebviewParams {
    fn default() -> Self {
        Self {
            output_gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Linear {
                    min: util::db_to_gain(-24.0),
                    max: util::db_to_gain(0.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(10.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            drive: FloatParam::new(
                "Drive",
                200.0,
                FloatRange::Skewed {
                    min: 100.0,
                    max: 10_000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(30.0))
            .with_unit("%"),
        }
    }
}

impl Plugin for UsingWebview {
    const NAME: &'static str = "UsingWebview";
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

        true
    }

    fn reset(&mut self) {}

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for mut channel_samples in buffer.iter_samples() {
            let output_gain = self.params.output_gain.smoothed.next();
            let drive = self.params.drive.smoothed.next();

            for sample in channel_samples.iter_mut() {
                *sample = drive::drive(sample.clone().into(), drive) * output_gain;
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for UsingWebview {
    const CLAP_ID: &'static str = "hectorbennett.using_webview";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("UsingWebviewooo");
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

impl Vst3Plugin for UsingWebview {
    const VST3_CLASS_ID: [u8; 16] = *b"UsingWebviewoooooooo";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Distortion];
}

nih_export_clap!(UsingWebview);
nih_export_vst3!(UsingWebview);
