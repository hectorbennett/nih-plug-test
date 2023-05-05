use atomic_float::AtomicF32;
use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui, widgets, EguiState};
use std::sync::Arc;

mod dial2;
mod toggle_switch;

/// The time it takes for the peak meter to decay by 12 dB after switching to complete silence.
const PEAK_METER_DECAY_MS: f64 = 150.0;

// FIXME: Theme should be `Copy` since it isn't big enough to generate a call to `memcpy`,
// do this when egui releases a minor version
/// The colors for a theme variant.
// #[derive(Debug, Clone, Hash, PartialEq, Eq)]
// pub struct Theme {
//     text: egui::Color32,

//     // noninteractive widgets
//     noninteractive_widget_bg_fill: egui::Color32,
//     // weak_bg_fill: noninteractive_widget_weak_bg_fill: egui::Color32,
//     noninteractive_widget_bg_stroke: egui::Color32,
//     noninteractive_widget_rounding: egui::Color32,
//     noninteractive_widget_fg_stroke: egui::Color32,
//     noninteractive_widget_expansion: egui::Color32,

//     // inactive widgets
//     inactive_widget_bg_fill: egui::Color32,
//     // weak_bg_fill: inactive_widget_weak_bg_fill: egui::Color32,
//     inactive_widget_bg_stroke: egui::Color32,
//      inactive_widget_rounding: egui::Color32,
//      inactive_widget_fg_stroke: egui::Color32,
//      inactive_widget_expansion: egui::Color32,

//     // hovered widgets
//      hovered_widget_bg_fill: egui::Color32,
//     // weak_bg_fill: hovered_widget_weak_bg_fill: egui::Color32,
//      hovered_widget_bg_stroke: egui::Color32,
//      hovered_widget_rounding: egui::Color32,
//      hovered_widget_fg_stroke: egui::Color32,
//      hovered_widget_expansion: egui::Color32,

//     // active widgets
//      active_widget_bg_fill: egui::Color32,
//     // weak_bg_fill: active_widget_weak_bg_fill: egui::Color32,
//      active_widget_bg_stroke: egui::Color32,
//      active_widget_rounding: egui::Color32,
//      active_widget_fg_stroke: egui::Color32,
//      active_widget_expansion: egui::Color32,

//     // open widgets
//      open_widget_bg_fill: egui::Color32,
//     // weak_bg_fill: open_widget_weak_bg_fill: egui::Color32,
//      open_widget_bg_stroke: egui::Color32,
//      open_widget_rounding: egui::Color32,
//      open_widget_fg_stroke: egui::Color32,
//      open_widget_expansion: egui::Color32,

// pub rosewater: egui::Color32,
// pub flamingo: egui::Color32,
// pub pink: egui::Color32,
// pub mauve: egui::Color32,
// pub red: egui::Color32,
// pub maroon: egui::Color32,
// pub peach: egui::Color32,
// pub yellow: egui::Color32,
// pub green: egui::Color32,
// pub teal: egui::Color32,
// pub sky: egui::Color32,
// pub sapphire: egui::Color32,
// pub blue: egui::Color32,
// pub lavender: egui::Color32,
// pub text: egui::Color32,
// pub subtext1: egui::Color32,
// pub subtext0: egui::Color32,
// pub overlay2: egui::Color32,
// pub overlay1: egui::Color32,
// pub overlay0: egui::Color32,
// pub surface2: egui::Color32,
// pub surface1: egui::Color32,
// pub surface0: egui::Color32,
// pub base: egui::Color32,
// pub mantle: egui::Color32,
// pub crust: egui::Color32,
// }

// pub const DEFAULT_THEME: Theme = Theme {
//     text: egui::Color32::from_rgb(76, 79, 105),

// rosewater: egui::Color32::from_rgb(220, 138, 120),
// flamingo: egui::Color32::from_rgb(221, 120, 120),
// pink: egui::Color32::from_rgb(234, 118, 203),
// mauve: egui::Color32::from_rgb(136, 57, 239),
// red: egui::Color32::from_rgb(210, 15, 57),
// maroon: egui::Color32::from_rgb(230, 69, 83),
// peach: egui::Color32::from_rgb(254, 100, 11),
// yellow: egui::Color32::from_rgb(223, 142, 29),
// green: egui::Color32::from_rgb(64, 160, 43),
// teal: egui::Color32::from_rgb(23, 146, 153),
// sky: egui::Color32::from_rgb(4, 165, 229),
// sapphire: egui::Color32::from_rgb(32, 159, 181),
// blue: egui::Color32::from_rgb(30, 102, 245),
// lavender: egui::Color32::from_rgb(114, 135, 253),
// subtext1: egui::Color32::from_rgb(92, 95, 119),
// subtext0: egui::Color32::from_rgb(108, 111, 133),
// overlay2: egui::Color32::from_rgb(124, 127, 147),
// overlay1: egui::Color32::from_rgb(140, 143, 161),
// overlay0: egui::Color32::from_rgb(156, 160, 176),
// surface2: egui::Color32::from_rgb(172, 176, 190),
// surface1: egui::Color32::from_rgb(188, 192, 204),
// surface0: egui::Color32::from_rgb(204, 208, 218),
// base: egui::Color32::from_rgb(239, 241, 245),
// mantle: egui::Color32::from_rgb(230, 233, 239),
// crust: egui::Color32::from_rgb(220, 224, 232),
// };

// pub fn make_widget_visuals(old_widget: egui::style::WidgetVisuals, theme: &Theme) -> egui::style::WidgetVisuals {
//     old_widget
// }

/// Apply the given theme to a [`Context`](egui::Context).
pub fn set_theme(ctx: &egui::Context) {
    let old = ctx.style().visuals.clone();
    ctx.set_visuals(egui::Visuals {
        override_text_color: old.override_text_color,
        widgets: egui::style::Widgets {
            noninteractive: egui::style::WidgetVisuals {
                bg_stroke: egui::Stroke {
                    color: egui::Color32::from_rgb(0, 255, 0),
                    width: 1.0,
                },
                ..old.widgets.noninteractive
            },
            inactive: egui::style::WidgetVisuals {
                bg_stroke: egui::Stroke {
                    color: egui::Color32::from_rgb(0, 255, 0),
                    width: 1.0,
                },
                ..old.widgets.inactive
            },
            hovered: egui::style::WidgetVisuals {
                bg_stroke: egui::Stroke {
                    color: egui::Color32::from_rgb(0, 255, 0),
                    width: 1.0,
                },
                ..old.widgets.hovered
            },
            active: egui::style::WidgetVisuals {
                bg_stroke: egui::Stroke {
                    color: egui::Color32::from_rgb(0, 255, 0),
                    width: 1.0,
                },
                ..old.widgets.active
            },
            open: egui::style::WidgetVisuals {
                bg_stroke: egui::Stroke {
                    color: egui::Color32::from_rgb(0, 255, 0),
                    width: 1.0,
                },
                ..old.widgets.open
            },
        },
        selection: egui::style::Selection { ..old.selection },
        // hyperlink_color: theme.hyperlink_color,
        // faint_bg_color: theme.faint_bg_color,
        // extreme_bg_color: theme.extreme_bg_color,
        // code_bg_color: theme.code_bg_color,
        // warn_fg_color: theme.warn_fg_color,
        // error_fg_color: theme.error_fg_color,
        // window_rounding
        // window_shadow,
        // window_fill: theme.base,
        // window_stroke: egui::Stroke {
        //     color: theme.overlay1,
        //     ..old.window_stroke
        // },
        // menu_rounding
        // panel_fill
        // popup_shadow
        // resize_corner_size
        // text_cursor_width
        // text_cursor_preview
        // clip_rect_margin
        // button_frame
        // collapsing_header_frame
        // indent_has_left_vline
        // striped
        // slider_trailing_fill

        // panel_fill: theme.base,

        // window_shadow: epaint::Shadow {
        //     color: theme.base,
        //     ..old.window_shadow
        // },
        // popup_shadow: epaint::Shadow {
        //     color: theme.base,
        //     ..old.popup_shadow
        // },
        ..old
    });
}

/// This is mostly identical to the gain example, minus some fluff, and with a GUI.
pub struct Gain {
    params: Arc<GainParams>,

    /// Needed to normalize the peak meter's response based on the sample rate.
    peak_meter_decay_weight: f32,
    /// The current data for the peak meter. This is stored as an [`Arc`] so we can share it between
    /// the GUI and the audio processing parts. If you have more state to share, then it's a good
    /// idea to put all of that in a struct behind a single `Arc`.
    ///
    /// This is stored as voltage gain.
    peak_meter: Arc<AtomicF32>,
}

#[derive(Params)]
pub struct GainParams {
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    editor_state: Arc<EguiState>,

    #[id = "gain"]
    pub gain: FloatParam,

    // TODO: Remove this parameter when we're done implementing the widgets
    #[id = "foobar"]
    pub some_int: IntParam,
}

impl Default for Gain {
    fn default() -> Self {
        Self {
            params: Arc::new(GainParams::default()),

            peak_meter_decay_weight: 1.0,
            peak_meter: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),
        }
    }
}

impl Default for GainParams {
    fn default() -> Self {
        Self {
            // set window size
            editor_state: EguiState::from_size(500, 500),

            // See the main gain example for more details
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            some_int: IntParam::new("Something", 3, IntRange::Linear { min: 0, max: 3 }),
        }
    }
}

impl Plugin for Gain {
    const NAME: &'static str = "Styling egui";
    const VENDOR: &'static str = "Moist Plugins GmbH";
    const URL: &'static str = "https://youtu.be/dQw4w9WgXcQ";
    const EMAIL: &'static str = "info@example.com";

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

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let params = self.params.clone();
        let peak_meter = self.peak_meter.clone();
        create_egui_editor(
            // State
            self.params.editor_state.clone(),
            // User state
            (),
            // Build
            |egui_ctx, _state| {
                set_theme(egui_ctx);
            },
            // Update
            move |egui_ctx, setter, _state| {
                egui::CentralPanel::default().show(egui_ctx, |ui| {
                    // NOTE: See `plugins/diopser/src/editor.rs` for an example using the generic UI widget

                    // This is a fancy widget that can get all the information it needs to properly
                    // display and modify the parameter from the parametr itself
                    // It's not yet fully implemented, as the text is missing.
                    ui.label("Some random integer");
                    ui.add(widgets::ParamSlider::for_param(&params.some_int, setter));

                    ui.label("Gain");
                    ui.add(widgets::ParamSlider::for_param(&params.gain, setter));

                    ui.label("Dial");
                    ui.add(dial2::Dial::for_param(&params.gain, setter));

                    ui.label(
                        "Also gain, but with a lame widget. Can't even render the value correctly!",
                    );
                    // This is a simple naieve version of a parameter slider that's not aware of how
                    // the parameters work
                    ui.add(
                        egui::widgets::Slider::from_get_set(-30.0..=30.0, |new_value| {
                            match new_value {
                                Some(new_value_db) => {
                                    let new_value = util::gain_to_db(new_value_db as f32);

                                    setter.begin_set_parameter(&params.gain);
                                    setter.set_parameter(&params.gain, new_value);
                                    setter.end_set_parameter(&params.gain);

                                    new_value_db
                                }
                                None => util::gain_to_db(params.gain.value()) as f64,
                            }
                        })
                        .suffix(" dB"),
                    );

                    // TODO: Add a proper custom widget instead of reusing a progress bar
                    let peak_meter =
                        util::gain_to_db(peak_meter.load(std::sync::atomic::Ordering::Relaxed));
                    let peak_meter_text = if peak_meter > util::MINUS_INFINITY_DB {
                        format!("{peak_meter:.1} dBFS")
                    } else {
                        String::from("-inf dBFS")
                    };

                    let mut boolean = false;

                    ui.add(toggle_switch::toggle(&mut boolean)).on_hover_text(
                        "It's easy to create your own widgets!\n\
                        This toggle switch is just 15 lines of code.",
                    );

                    // ui.add(dial::dial(-30.0, 30.0, &params.gain)).on_hover_text(
                    //     "It's easy to create your own widgets!\n\
                    //     This toggle switch is just 15 lines of code.",
                    // );

                    // ui.add(dial::for_param(&params.gain, setter));

                    let peak_meter_normalized = (peak_meter + 60.0) / 60.0;
                    ui.allocate_space(egui::Vec2::splat(2.0));
                    ui.add(
                        egui::widgets::ProgressBar::new(peak_meter_normalized)
                            .text(peak_meter_text),
                    );
                });
            },
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // After `PEAK_METER_DECAY_MS` milliseconds of pure silence, the peak meter's value should
        // have dropped by 12 dB
        self.peak_meter_decay_weight = 0.25f64
            .powf((buffer_config.sample_rate as f64 * PEAK_METER_DECAY_MS / 1000.0).recip())
            as f32;

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let mut amplitude = 0.0;
            let num_samples = channel_samples.len();

            let gain = self.params.gain.smoothed.next();
            for sample in channel_samples {
                *sample *= gain;
                amplitude += *sample;
            }

            // To save resources, a plugin can (and probably should!) only perform expensive
            // calculations that are only displayed on the GUI while the GUI is open
            if self.params.editor_state.is_open() {
                amplitude = (amplitude / num_samples as f32).abs();
                let current_peak_meter = self.peak_meter.load(std::sync::atomic::Ordering::Relaxed);
                let new_peak_meter = if amplitude > current_peak_meter {
                    amplitude
                } else {
                    current_peak_meter * self.peak_meter_decay_weight
                        + amplitude * (1.0 - self.peak_meter_decay_weight)
                };

                self.peak_meter
                    .store(new_peak_meter, std::sync::atomic::Ordering::Relaxed)
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for Gain {
    const CLAP_ID: &'static str = "com.moist-plugins-gmbh-egui.gain-gui";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A smoothed gain parameter example plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for Gain {
    const VST3_CLASS_ID: [u8; 16] = *b"GainGuiYeahBoyyy";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_clap!(Gain);
nih_export_vst3!(Gain);
