use crate::app::GraphApp;

mod app;
mod graph;
mod painter;
mod traversers;
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Graph Visualizer",
        native_options,
        Box::new(|cc| Box::new(GraphApp::new(cc))),
    )
}
