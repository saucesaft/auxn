use atomic_float::AtomicF32;
use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui, EguiState};
use std::sync::{Arc, Mutex};
use std::{thread, time, mem};
use std::sync::mpsc;

mod uxn;
mod system;
mod devices;
mod operations;

use uxn::UXN;
use devices::DrawOperation;

/// The time it takes for the peak meter to decay by 12 dB after switching to complete silence.
const PEAK_METER_DECAY_MS: f64 = 150.0;

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
            editor_state: EguiState::from_size(WIDTH, HEIGHT),

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

fn loopy() {
    println!("another thread :)");
}

impl Plugin for Gain {
    const NAME: &'static str = "Gain GUI (egui)";
    const VENDOR: &'static str = "Moist Plugins GmbH";
    const URL: &'static str = "https://youtu.be/dQw4w9WgXcQ";
    const EMAIL: &'static str = "info@example.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const DEFAULT_INPUT_CHANNELS: u32 = 2;
    const DEFAULT_OUTPUT_CHANNELS: u32 = 2;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type BackgroundTask = &'static (dyn Fn() + Sync + Send);

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&self, async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {

        async_executor.execute_gui(&loopy);

        let (sender, receiver): (mpsc::Sender<devices::DrawOperation>, mpsc::Receiver<devices::DrawOperation>) = mpsc::channel();
        let rx = Mutex::new(receiver);

        let uxn = Mutex::new(UXN::new(WIDTH, HEIGHT, sender));

        {
            // let rom = include_bytes!("../pixel.rom").to_vec();
            let rom = include_bytes!("../../uxn/line.rom").to_vec();
            // let rom = include_bytes!("../../uxn/pixelframe.rom").to_vec();

            let mut setup = uxn.lock().unwrap();
            setup.pc = 0x100;

            setup.load(rom);
        }

        let params = self.params.clone();
        let peak_meter = self.peak_meter.clone();
        create_egui_editor(
            self.params.editor_state.clone(),
            (),
            |_, _| {},
            move |egui_ctx, setter, _state| {
                egui::CentralPanel::default().show(egui_ctx, |ui| {
                    let painter = ui.painter();

                    let mut cycle = uxn.lock().unwrap();

                    if !cycle.halted {
                        cycle.step();
                    }

                    // new implementation idea,
                    // list of enum messages with color and x,y coordinate
                    // instead of vectors
                    let p1 = egui::Pos2::new(0.0, 0.0);
                    let p2 = egui::Pos2::new(WIDTH as f32, HEIGHT as f32);

                    let rect = egui::Rect::from_two_pos(p1, p2);

                    let color = cycle.system.get_color(0);

                    painter.rect_filled(
                        rect,
                        0.0,
                        color,
                    );

                    while let Ok(draw_op) = rx.lock() .unwrap().try_recv() {
                        match draw_op {
                            DrawOperation::Pixel{x, y, color} => {
                                let pos1 = egui::Pos2::new(x as f32, y as f32);
                                let pos2 = egui::Pos2::new((x+1) as f32, (y+1) as f32);

                                let rect = egui::Rect::from_two_pos(pos1, pos2);

                                painter.rect_filled(
                                    rect,
                                    0.0,
                                    color,
                                );
                            }
                        }
                    }

                    // for (i, el) in cycle.screen.fg.iter().enumerate() {
                    //     if *el != -1 {
                    //         // UPDATE THE WIDTH, NOT ONLY THE CONSTANT
                    //         let x = (i as u32) % WIDTH;
                    //         let y = (i as u32) / WIDTH;

                    //         let p1 = egui::Pos2::new(x as f32, y as f32);
                    //         let p2 = egui::Pos2::new((x+1) as f32, (y+1) as f32);

                    //         let rect = egui::Rect::from_two_pos(p1, p2);

                    //         let color = cycle.system.get_color(*el);

                    //         painter.rect_filled(
                    //             rect,
                    //             0.0,
                    //             color,
                    //         );

                    //     }
                    // }

                });
            },
        )
    }

    // fn pixel(&self, x, y)

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