use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(480.0, 320.0))
            .with_min_inner_size(egui::vec2(360.0, 240.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Hell World 🔥",
        options,
        Box::new(|_cc| Ok(Box::new(HellWorldApp::default()))),
    )
}

#[derive(Default)]
struct HellWorldApp {
    clicks: u32,
}

impl eframe::App for HellWorldApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("
                🔥 Welcome to Hell World");
                ui.label("The underflow is warm this time of year...");

                if ui.button("Summon a demon").clicked() {
                    self.clicks += 1;
                }
                if self.clicks > 0 {
                    ui.label(format!("You have summoned {} demons 😈", self.clicks));
                }
            });
        });
    }
}
