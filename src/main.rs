// Entry point desktop (untuk uji cepat: `cargo run`)
use hello_rust_android::MyApp;

fn main() -> eframe::Result {
    eframe::run_native(
        "Hello Rust",
        Default::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
