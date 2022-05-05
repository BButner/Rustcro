use std::time::Duration;
use eframe::{egui, Storage};
use eframe::emath::Vec2;
use eframe::epaint::Rgba;
use eframe::glow::Context;
use egui::{FontFamily, TextEdit, Visuals, RichText, FontId, Button};
use crate::config::Config;

pub fn launch_gui(config: Config) {
    let options = eframe::NativeOptions {
        transparent: true,
        decorated: false,
        initial_window_size: Option::from(Vec2 { x: 500.0, y: 200.0 }),
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
        egui::TopBottomPanel::top("")
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                let text_len = self.current_text.len();
                let text = &self.current_text.clone();

                let textedit =
                    TextEdit::singleline(&mut self.current_text)
                        .desired_width(500.0)
                        .hint_text("Macro Key")
                        .font(FontId { size: 32.0, family: Default::default() });

                ui.shrink_height_to_current();
                ui.shrink_width_to_current();
                ui.vertical(|ui| {
                    ui.label("Testing");
                    ui.add(textedit);

                    if text_len > 1 {
                        println!("testing");
                        println!("{:?}", self.config.typing_macros);

                        &for x in self.config.typing_macros
                            .iter()
                            .filter(|m| m.key.contains(text)) {
                            ui.add(Button::new(&x.key));
                        };
                    }
                });
            });
    }
}