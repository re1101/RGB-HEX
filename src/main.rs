use eframe::egui;
use std::sync::Arc;

fn main() -> Result<(), eframe::Error> {
    //let icon = image::open("res/icon.ico").expect("Failed to open icon path").to_rgba8();
    //let (icon_width, icon_height) = icon.dimensions();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            title: Some("RGB-HEX Converter".to_owned()),
            min_inner_size: Some(egui::vec2(400.0, 300.0)),
            max_inner_size: Some(egui::vec2(1000.0, 600.0)),
            icon: Some(Arc::new(egui::IconData{
                rgba: /*icon.into_raw()*/ [0,0,0,255].to_vec(),
                width: /*icon_width*/ 1,
                height: /*icon_height*/ 1,
            })),
            taskbar: Some(true),
            resizable: Some(true),
            ..Default::default()
        },
        ..Default::default()
    };
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
            ui.label(format!("{:?}", ui.unique_id()));

            ui.heading("RGB-HEX Converter");

            ui.horizontal(|ui| {
                ui.label("RGB:");
                for i in 0..3 {
                    ui.add(egui::DragValue::new(&mut self.rgb[i]).range(0..=255));
                }
            });

            self.hex = rgb_hex(self.rgb[0], self.rgb[1], self.rgb[2]);

            ui.horizontal(|ui| {
                ui.label("HEX:");
                ui.text_edit_singleline(&mut self.hex);
            });

            let (r, g, b) = hex_rgb(self.hex.clone());
            self.rgb = [r, g, b];

            ui.strong("Color Preview:");

            ui.add_sized( [40.0 ,20.0], &mut egui::Frame {
                fill: egui::Color32::from_rgba_unmultiplied(
                    self.rgb[0],
                    self.rgb[1],
                    self.rgb[2],
                    255,
                ),
                ..Default::default()
            });

            ctx.send_viewport_cmd(egui::ViewportCommand::Icon(Some(Arc::new(
                egui::IconData {
                    rgba: [self.rgb[0], self.rgb[1], self.rgb[2], 255].to_vec(),
                    width: 1,
                    height: 1,
                },
            ))));
        });
    }
}
fn rgb_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn hex_rgb(hex: String) -> (u8, u8, u8) {
    let mut hex_aux: String = hex.trim_start_matches('#').to_owned();
    if hex_aux.len() != 6 {
        if hex_aux.len() < 6{
            while hex_aux.len() < 6{
                hex_aux.push('0');
            }
        } else{
            hex_aux = hex_aux[0..6].to_owned();
        }
    }
    match (
        u8::from_str_radix(&hex_aux[0..2], 16),
        u8::from_str_radix(&hex_aux[2..4], 16),
        u8::from_str_radix(&hex_aux[4..6], 16),
    ) {
        (Ok(r), Ok(g), Ok(b)) => (r, g, b),
        _ => (0, 0, 0),
    }
}
