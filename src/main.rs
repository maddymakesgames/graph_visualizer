#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
pub mod generation;
mod graph;
pub mod menus;
mod painter;
mod traversers;

use crate::app::AppManager;
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    tracing_subscriber::fmt::init();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Graph Visualizer",
        native_options,
        Box::new(|cc| Box::new(AppManager::new(cc))),
    )
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "the_canvas_id", // hardcode it
        web_options,
        Box::new(|cc| Box::new(AppManager::new(cc))),
    )
    .expect("failed to start eframe");
}
