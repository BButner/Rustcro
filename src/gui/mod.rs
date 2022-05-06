use std::time::Duration;
use eframe::{egui, Storage};
use eframe::emath::Vec2;
use eframe::epaint::{Color32, Rgba};
use eframe::glow::Context;
use egui::{FontFamily, TextEdit, Visuals, RichText, FontId, Button, Stroke, Frame};
use crate::config::Config;
use crate::macros;
use crate::macros::run_macro;

const WINDOW_WIDTH: f32 = 500.0;

pub fn launch_gui(config: Config) {
    let options = eframe::NativeOptions {
        decorated: false,
        transparent: true,
        initial_window_size: Option::from(Vec2::new(500., 100.)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        "Rustcro",
        options,
        Box::new(|_cc| Box::new(MacroBar {
            current_text: String::new(),
            config,
        })),
    );
}

struct MacroBar {
    current_text: String,
    config: Config,
}

impl eframe::App for MacroBar {
    fn clear_color(&self, _visuals: &Visuals) -> Rgba {
        Rgba::TRANSPARENT
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame {
                fill: Color32::TRANSPARENT,
                ..Default::default()
            })
            .show(ctx, |ui| {
                let text_len = self.current_text.len();
                let text = &self.current_text.clone();

                ui.vertical_centered(|ui| {
                    let textedit =
                        TextEdit::singleline(&mut self.current_text)
                            .desired_width(500.0)
                            .hint_text("Macro Key")
                            .font(FontId { size: 32.0, family: Default::default() });

                    ui.add(textedit);

                    if text_len > 1 {
                        &for x in self.config.typing_macros
                            .iter()
                            .filter(|m| m.key.contains(text))
                            .take(5) {
                            let button = Button::new(&x.key)
                                .stroke(Stroke { width: 0.0, color: Default::default() });

                            if ui.add_sized(Vec2::new(500., 20.), button).clicked() {
                                frame.quit();
                                run_macro(&x);
                            }
                        };
                    }
                });
            });

        frame.set_window_size(ctx.used_size());
    }
}