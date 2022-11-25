pub mod egui_settings;
pub mod graphs;
pub mod nodes;
pub mod painter;
pub mod traversals;

use egui::Ui;

use crate::{
    app::GraphApp,
    graph::Graph,
    menus::{
        egui_settings::EguiSettings, graphs::GraphMenu, nodes::NodesMenu,
        painter::GraphPainterMenu, traversals::TraversalMenu,
    },
};

pub const MENU_COUNT: usize = 5;

pub fn menus() -> [Box<dyn Menu>; MENU_COUNT] {
    [
        Box::new(GraphMenu::default()),
        Box::new(NodesMenu::default()),
        Box::new(TraversalMenu::default()),
        Box::new(GraphPainterMenu),
        Box::new(EguiSettings),
    ]
}

pub trait Menu {
    fn ui(&mut self, app: &mut GraphApp, ui: &mut Ui);

    fn name(&self) -> &'static str;

    // When RA autocompletes a trait function
    // it defaults the parameters to the ones in the definition
    // so we don't want to mark this as unused
    #[allow(unused)]
    fn graph_updated(&mut self, graph: &Graph) {}
}
