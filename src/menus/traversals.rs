use std::collections::HashMap;

use egui::{ComboBox, DragValue, Ui, Widget};

use crate::{app::GraphApp, menus::Menu, traversers::GraphTraversers};

#[derive(Default)]
pub struct TraversalMenu {
    pub start_node: usize,
    pub end_node: usize,
    pub debug_view: bool,
}

impl Menu for TraversalMenu {
    fn ui(&mut self, app: &mut GraphApp, ui: &mut Ui) {
        if let Some(graph) = app.graphs.get_mut(app.curr_graph) {
            let manager = &mut app.traversal_manager;
            let algs = GraphTraversers::values();
            let mut curr_alg = algs.iter().position(|a| *a == manager.alg).unwrap();

            ComboBox::from_label("Traversal Algorithm").show_index(
                ui,
                &mut curr_alg,
                algs.len(),
                |i| algs[i].name().to_owned(),
            );

            manager.alg = algs[curr_alg];

            let nodes = graph.get_nodes();

            let idx_to_name = nodes
                .iter()
                .map(|n| (n.get_id(), n.get_name()))
                .collect::<HashMap<_, _>>();
            let usize_to_idx = nodes.iter().map(|n| n.get_id()).collect::<Vec<_>>();

            ComboBox::from_label("Start Node").show_index(
                ui,
                &mut self.start_node,
                usize_to_idx.len() + 1,
                |i| {
                    if i == 0 {
                        "Select Node".to_owned()
                    } else {
                        (*idx_to_name.get(&usize_to_idx[i - 1]).unwrap()).to_owned()
                    }
                },
            );

            ComboBox::from_label("End Node").show_index(
                ui,
                &mut self.end_node,
                usize_to_idx.len() + 1,
                |i| {
                    if i == 0 {
                        "Select Node".to_owned()
                    } else {
                        (*idx_to_name.get(&usize_to_idx[i - 1]).unwrap()).to_owned()
                    }
                },
            );

            ui.checkbox(&mut manager.auto, "Automatically Traverse");

            if manager.auto {
                ui.horizontal(|ui| {
                    ui.label("Milliseconds between steps");
                    DragValue::new(&mut manager.speed)
                        .clamp_range(0..=20)
                        .ui(ui);
                });
            } else if manager.currently_traversing && ui.button("Step Traversal").clicked() {
                manager.update(graph);
            }

            if manager.traversal.is_some() {
                if ui.button("Stop Traversal").clicked() {
                    manager.traversal = None;
                    graph.reset();
                }
            } else {
                // Isn't collapsible because the button call has side effects
                // You're not supposed to do this but /shrug
                #[allow(clippy::collapsible_else_if)]
                if ui.button("Start Traversal").clicked()
                    && self.start_node > 0
                    && self.end_node > 0
                {
                    manager.new_traversal(
                        usize_to_idx[self.start_node - 1],
                        usize_to_idx[self.end_node - 1],
                    );
                }
            }

            ui.checkbox(&mut self.debug_view, "Debug View");

            if self.debug_view {
                if let Some(traversal) = &manager.traversal {
                    ui.horizontal(|ui| {
                        ui.label("To Traverse: ");
                        ui.monospace(format!(
                            "{:?}",
                            traversal
                                .to_traverse
                                .iter()
                                .map(|(_, id)| graph.get_node(*id).get_name())
                                .collect::<Vec<_>>()
                        ))
                    });

                    ui.horizontal(|ui| {
                        ui.label("Visited: ");
                        ui.monospace(format!(
                            "{:?}",
                            traversal
                                .visited
                                .iter()
                                .map(|id| graph.get_node(*id).get_name())
                                .collect::<Vec<_>>()
                        ))
                    });
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "Traversal"
    }

    fn graph_updated(&mut self, _graph: &crate::graph::Graph) {
        self.start_node = 0;
        self.end_node = 0;
    }
}
