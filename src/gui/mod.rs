use eframe::{egui};
use eframe::emath::Vec2;
use eframe::epaint::{Color32, Rgba};
use egui::{TextEdit, Visuals, FontId, Button, Stroke, Frame};
use crate::config::Config;
use crate::macros::run_macro;

pub fn launch_gui(config: Config) {
    let options = eframe::NativeOptions {
        decorated: false,
        transparent: true,
        initial_window_size: Option::from(Vec2::new(500., 200.)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        "Rustcro",
        options,
        Box::new(|_cc| Box::new(MacroBar {
            current_text: String::new(),
            config,
            initial_focus_set: false,
        })),
    );
}

struct MacroBar {
    current_text: String,
    config: Config,
    initial_focus_set: bool,
}

impl eframe::App for MacroBar {
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
                            .hint_text("testing")
                            .font(FontId { size: 32.0, family: Default::default() });

                    let textedit_response = ui.add(textedit);

                    if !self.initial_focus_set {
                        self.initial_focus_set = true;
                        textedit_response.request_focus();
                    }

                    if text_len > 1 {
                        let _ = &for x in self.config.macros
                            .iter()
                            .filter(|m| m.key.contains(text))
                            .take(5) {
                            if textedit_response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                frame.quit();
                                run_macro(&x);
                                break;
                            }

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
    }

    fn clear_color(&self, _visuals: &Visuals) -> Rgba {
        Rgba::TRANSPARENT
    }
}