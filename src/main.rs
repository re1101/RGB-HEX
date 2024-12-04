//use std::io;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "RGB-HEX Converter",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    rgb: [u8; 3],
    hex: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            rgb: [0, 0, 0],
            hex: String::from("#000000"),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("RGB-HEX Converter");

            ui.horizontal(|ui| {
                ui.label("RGB:");
                for i in 0..3 {
                    ui.add(egui::DragValue::new(&mut self.rgb[i]).range(0..=255));
                }
            });

            if ui.button("RGB to HEX").clicked() {
                self.hex = rgb_hex(self.rgb[0], self.rgb[1], self.rgb[2]);
            }

            ui.horizontal(|ui| {
                ui.label("HEX:");
                ui.text_edit_singleline(&mut self.hex);
            });

            if ui.button("HEX to RGB").clicked() {
                let (r, g, b) = hex_rgb(self.hex.clone());
                self.rgb = [r, g, b];
            }

            ui.colored_label(
                egui::Color32::from_rgb(self.rgb[0], self.rgb[1], self.rgb[2]),
                "Color Preview",
            );
        });
    }
}
fn rgb_hex(r: u8, g: u8, b: u8) -> String{
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn hex_rgb(hex: String) -> (u8, u8, u8){
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return (0, 0, 0);
    }
    match (
        u8::from_str_radix(&hex[0..2], 16),
        u8::from_str_radix(&hex[2..4], 16),
        u8::from_str_radix(&hex[4..6], 16),
    ) {
        (Ok(r), Ok(g), Ok(b)) => (r, g, b),
        _ => (0, 0, 0),
    }
}
