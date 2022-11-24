use crate::app::AppManager;

mod app;
mod graph;
pub mod menus;
mod painter;
mod traversers;
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Graph Visualizer",
        native_options,
        Box::new(|cc| Box::new(AppManager::new(cc))),
    )
}
