use atomic_float::AtomicF32;

use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui, EguiState};

use egui_memory_editor::MemoryEditor;

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::{mem, thread, time};

mod devices;
mod operations;
mod system;
mod uxn;

use uxn::UXN;

/// The time it takes for the peak meter to decay by 12 dB after switching to complete silence.
const PEAK_METER_DECAY_MS: f64 = 150.0;

// 512 * 320

const WIDTH: u32 = 64 * 8;
const HEIGHT: u32 = 40 * 8;

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
            // editor_state: EguiState::from_size(WIDTH, HEIGHT),
            editor_state: EguiState::from_size(1000, 600),

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
    const NAME: &'static str = "talsnd";
    const VENDOR: &'static str = "auxsaft";
    const URL: &'static str = "google.com";
    const EMAIL: &'static str = "eduarch42@protonmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const DEFAULT_INPUT_CHANNELS: u32 = 2;
    const DEFAULT_OUTPUT_CHANNELS: u32 = 2;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type BackgroundTask = &'static (dyn Fn() + Sync + Send);

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&self, async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let uxn = Mutex::new(UXN::new(WIDTH, HEIGHT));

        {
            // general tests //
            // let rom = include_bytes!("../tests/arithmetic.rom").to_vec();
            // let rom = include_bytes!("../tests/literals.rom").to_vec();
            // let rom = include_bytes!("../tests/jumps.rom").to_vec();
            // let rom = include_bytes!("../tests/memory.rom").to_vec();
            // let rom = include_bytes!("../tests/stack.rom").to_vec();
            // let rom = include_bytes!("../tests.rom").to_vec();

            // video related //
            // let rom = include_bytes!("../pixel.rom").to_vec();
            // let rom = include_bytes!("../../uxn/line.rom").to_vec();
            // let rom = include_bytes!("../../uxn/pixelframe.rom").to_vec();
            
            // demos //
            let rom = include_bytes!("../../uxn/sprite_test.rom").to_vec();
            // let rom = include_bytes!("../../uxn/amiga.rom").to_vec();
            // let rom = include_bytes!("../../uxn/polycat.rom").to_vec();
            // let rom = include_bytes!("../../uxn/dvd.rom").to_vec();

            let mut setup = uxn.lock().unwrap();

            setup.load(rom);

            // make it so that if any error happens
            // this function returns it and passes it to the gui

            // - maybe move this function to the init?
            // i have an slight asumption this will run everytime you open the gui
            // - run it as another thread but have it return a bool
            // that will change when the start is ready but let the app
            // advance into the gui and show that it is booting up
            setup.eval(0x100);

            // we set the new texture size, depending
            // if yes or not the user configured a custom size
            setup.resize();

            // we also asume the user did define the system colors
            // so we set the background color here
            setup.bg_color();
        }

        let params = self.params.clone();
        let peak_meter = self.peak_meter.clone();

        let memory_widget = Mutex::new(
            MemoryEditor::new().with_address_range("All", 0..0x13000)
            );

        {
            let mut mw_setup = memory_widget.lock().unwrap();
            mw_setup.options.show_ascii = false;
        }

        create_egui_editor(
            self.params.editor_state.clone(),
            (),
            move |_, _| {},
            move |ctx, setter, _state| {

                egui::CentralPanel::default().show(ctx, |ui| {

                    {
                        let mut cycle = uxn.lock().unwrap();
                        let mut hex = memory_widget.lock().unwrap();              

                        hex.window_ui_read_only(
                            ctx,
                            &mut true,
                            &mut cycle.ram,
                            |mem, addr| {
                                mem[addr].into()
                            },
                        );
                    }

                    egui::Window::new("uxn")
                    .show(ctx, |ui| {
                        let mut cycle = uxn.lock().unwrap();

                        let screen_vector_addr = cycle.screen.vector();

                        // return a result
                        // if we have an error, show an specific gui
                        cycle.eval(screen_vector_addr);

                        if cycle.screen.redraw {
                            cycle.screen.generate(ctx);
                            cycle.screen.redraw = false;
                        }

                        let texture = cycle.screen.display.as_ref().expect("No Texture Loaded");

                        ui.image(texture, texture.size_vec2());

                    });

                    // egui::Window::new("debug")
                    // .anchor(egui::Align2::CENTER_BOTTOM, egui::Vec2::new(0.0, -20.0))
                    // .show(ctx, |ui| {
                    //     ctx.texture_ui(ui);
                    // });

                });

            },
        )
    }

    fn accepts_bus_config(&self, config: &BusConfig) -> bool {
        // This works with any symmetrical IO layout
        config.num_input_channels == config.num_output_channels && config.num_input_channels > 0
    }

    fn initialize(
        &mut self,
        _bus_config: &BusConfig,
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
    const VST3_CATEGORIES: &'static str = "Fx|Dynamics";
}

nih_export_clap!(Gain);
nih_export_vst3!(Gain);
