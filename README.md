# Hello Rust Android

Aplikasi Android sederhana yang ditulis dengan Rust + egui/eframe.

## Build lewat GitHub
1. Push proyek ini ke repo GitHub (branch `main`).
2. Buka tab **Actions** → workflow **Build Android APK** (jalan otomatis saat push).
3. Setelah hijau, unduh artifact **HelloRust-apk** → ekstrak → install `.apk` ke HP.

## Uji di desktop
    cargo run

## Build lokal
    rustup target add aarch64-linux-android
    cargo install cargo-apk
    cargo apk build --release --lib
