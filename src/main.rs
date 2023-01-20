mod app;
mod keyfinder;

use eframe::epaint::Vec2;

use crate::{
    app::KeyFinderApplication,
    keys::{init, Key},
};

mod keys;

fn main() {
    // Initialize all - circle of fifths - key data
    let mut key_data: Vec<Key> = Vec::new();
    init(&mut key_data);

    // Egui
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(500., 750.)),
        resizable: false,
        ..Default::default()
    };

    eframe::run_native(
        "KeyFinder",
        native_options,
        Box::new(|cc| Box::new(KeyFinderApplication::new(cc))),
    );
}
