pub mod graphs;
pub mod nodes;
pub mod painter;
pub mod traversals;

use egui::Ui;

use crate::graph::Graph;

pub trait Menu {
    type NeededData;

    fn ui(&mut self, app: &mut Self::NeededData, ui: &mut Ui);

    fn name(&self) -> &'static str;

    fn graph_updated(&mut self, graph: &Graph);
}
