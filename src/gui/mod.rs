use std::time::Duration;
use eframe::{egui, Storage};
use eframe::emath::Vec2;
use eframe::epaint::{Color32, Rgba};
use eframe::glow::Context;
use egui::{FontFamily, TextEdit, Visuals, RichText, FontId, Button, Stroke};
use crate::config::Config;
use crate::macros::type_macro;

const WINDOW_WIDTH: f32 = 500.0;
const BUTTON_HEIGHT: f32 = 20.0;
const TEXT_HEIGHT: f32 = 36.0;

pub fn launch_gui(config: Config) {
    let options = eframe::NativeOptions {
        transparent: true,
        decorated: false,
        initial_window_size: Option::from(Vec2 { x: WINDOW_WIDTH, y: 80.0 }),
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
    // fn clear_color(&self, _visuals: &Visuals) -> Rgba {
    //     Rgba::TRANSPARENT
    // }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                let text_len = self.current_text.len();
                let text = &self.current_text.clone();
                let mut additional_height = 0.0;

                let textedit =
                    TextEdit::singleline(&mut self.current_text)
                        .desired_width(500.0)
                        .hint_text("Macro Key")
                        .font(FontId { size: 32.0, family: Default::default() });

                ui.vertical(|ui| {
                    ui.add(textedit);

                    if text_len > 1 {
                        &for x in self.config.typing_macros
                            .iter()
                            .filter(|m| m.key.contains(text))
                            .take(5) {
                            let button = Button::new(&x.key)
                                .stroke(Stroke { width: 0.0, color: Default::default() });

                            additional_height += BUTTON_HEIGHT;

                            if ui.add(button).clicked() {
                                frame.quit();
                                type_macro(&x.text);
                            }
                        };
                    }
                });

                // frame.set_window_size(Vec2::new(WINDOW_WIDTH, TEXT_HEIGHT + additional_height));
            });
    }
}