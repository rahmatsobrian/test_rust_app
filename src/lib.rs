use eframe::egui;

/// Titik masuk Android (dipanggil oleh NativeActivity).
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };
    eframe::run_native(
        "Hello Rust",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
    .unwrap();
}

pub struct MyApp {
    name: String,
    counter: i32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: String::new(),
            counter: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Ruang kosong supaya konten tidak tertutup status bar Android
        egui::Panel::top("status_bar_space").show(ui, |ui| {
            ui.set_height(36.0);
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(24.0);
                ui.heading("Halo dari Rust! 🦀");
                ui.add_space(16.0);

                ui.label("Siapa namamu?");
                ui.text_edit_singleline(&mut self.name);

                if !self.name.is_empty() {
                    ui.add_space(8.0);
                    ui.label(format!("Halo, {}!", self.name));
                }

                ui.add_space(24.0);
                ui.label(format!("Hitungan: {}", self.counter));
                ui.horizontal(|ui| {
                    if ui.button("  −  ").clicked() {
                        self.counter -= 1;
                    }
                    if ui.button("  +  ").clicked() {
                        self.counter += 1;
                    }
                    if ui.button("Reset").clicked() {
                        self.counter = 0;
                    }
                });
            });
        });
    }
}
