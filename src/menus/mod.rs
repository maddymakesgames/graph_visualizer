pub mod graphs;
pub mod nodes;
pub mod painter;
pub mod traversals;

use egui::Ui;

use crate::{app::GraphApp, graph::Graph};

pub trait Menu {
    fn ui(&mut self, app: &mut GraphApp, ui: &mut Ui);

    fn name(&self) -> &'static str;

    fn graph_updated(&mut self, graph: &Graph);
}
