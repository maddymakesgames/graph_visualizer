use egui::{ComboBox, DragValue, Ui, Widget, Window};
use rand::Rng;

use crate::{
    app::GraphApp,
    graph::{Graph, NodeIndex},
    menus::Menu,
    traversers::{GraphTraversers, TraversalManager},
};

#[derive(Default)]
pub struct GraphMenu {
    graph_name: String,
    directed: bool,
    weighted: bool,
    random_menu_open: bool,
    random_menu: RandomGraphMenu,
}

impl Menu for GraphMenu {
    fn ui(&mut self, graph_app: &mut GraphApp, ui: &mut Ui) {
        let curr_graph = graph_app.get_curr_graph_index();
        let graphs = graph_app.get_graphs();

        let mut graph_selection = if graphs.is_empty() { 0 } else { curr_graph + 1 };
        ComboBox::from_label("Select Graph").show_index(
            ui,
            &mut graph_selection,
            graphs.len() + 1,
            |i| {
                if i == 0 {
                    "Select Graph".to_owned()
                } else {
                    graphs[i - 1].get_name()
                }
            },
        );

        if graph_selection > 0 {
            graph_app.set_curr_graph_index(graph_selection - 1);
        }

        let graphs = graph_app.get_graphs();

        ui.horizontal(|ui| {
            ui.label("Graph Name");
            ui.text_edit_singleline(&mut self.graph_name);
        });

        ui.checkbox(&mut self.directed, "Directed Graph");
        ui.checkbox(&mut self.weighted, "Weighted Graph");

        if ui.button("Create Graph").clicked() {
            let name = std::mem::take(&mut self.graph_name);
            graphs.push(Graph::new(name, self.directed, self.weighted));
        }

        if ui.button("Create Random Graph").clicked() {
            self.random_menu_open = true;
        }

        if self.random_menu_open {
            if let Some(true) = Window::new("Random Graph Creation")
                .open(&mut self.random_menu_open)
                .show(ui.ctx(), |ui| self.random_menu.ui(ui))
                .map(|a| {
                    let b = a.inner;
                    matches!(b, Some(true))
                })
            {
                let graph = self.random_menu.gen_graph();

                graphs.push(graph);

                self.random_menu_open = false;
            }
        }
    }

    fn name(&self) -> &'static str {
        "Graphs"
    }
}

struct RandomGraphMenu {
    graph_name: String,
    connected: bool,
    directed: bool,
    node_count: u8,
    edge_count: u8,
    weights: bool,
    weight_lower_bound: f32,
    weight_upper_bound: f32,
}

impl RandomGraphMenu {
    fn ui(&mut self, ui: &mut Ui) -> bool {
        ui.horizontal(|ui| {
            ui.label("Graph Name");
            ui.text_edit_singleline(&mut self.graph_name);
        });

        ui.horizontal(|ui| {
            ui.label("Node Count");
            DragValue::new(&mut self.node_count)
                .clamp_range(1..=30)
                .ui(ui);
        });

        ui.horizontal(|ui| {
            ui.label("Edge Count");
            DragValue::new(&mut self.edge_count)
                .clamp_range(
                    if self.connected {
                        self.node_count as u32 - 1
                    } else {
                        0
                    }..=self.node_count as u32 * (self.node_count as u32 - 1),
                )
                .ui(ui);
        });

        ui.checkbox(&mut self.directed, "Directed Graph");
        ui.checkbox(&mut self.connected, "Connected Graph");
        ui.checkbox(&mut self.weights, "Weighted Graph");

        if self.weights {
            DragValue::new(&mut self.weight_lower_bound).ui(ui);

            if self.weight_upper_bound < self.weight_lower_bound {
                self.weight_upper_bound = self.weight_lower_bound;
            }

            DragValue::new(&mut self.weight_upper_bound)
                .clamp_range(self.weight_lower_bound..=f32::INFINITY)
                .ui(ui);
        }

        ui.button("Create Graph").clicked()
    }

    fn gen_graph(&self) -> Graph {
        let edge_count = self.edge_count;
        let node_count = self.node_count;

        let mut graph = Graph::new(self.graph_name.clone(), self.directed, self.weights);
        let mut rng = rand::thread_rng();

        for i in 0..node_count {
            let x = rng.gen_range(5.0..995.0);
            let y = rng.gen_range(5.0..995.0);
            graph.add_node((x, y), i.to_string(), Vec::new());
        }

        for i in 0..edge_count {
            // If we are making a connected graph, we ensure that each node gets at least 1 edge
            let a = if self.connected && i < node_count {
                i as usize
            } else {
                rng.gen_range(0..node_count) as usize
            };

            let mut b = rng.gen_range(0..node_count) as usize;

            while a == b && node_count > 1 {
                b = rng.gen_range(0..node_count) as usize;
            }

            let a = NodeIndex(a);
            let mut b = NodeIndex(b);

            while graph.get_node(a).get_edges().iter().any(|e| {
                let (c, d) = e.get_nodes();
                if c == a {
                    d == b
                } else if !self.directed {
                    c == b
                } else {
                    false
                }
            }) {
                b = NodeIndex(rng.gen_range(0..node_count) as usize);
            }

            let weight = if self.weights {
                Some(rng.gen_range(self.weight_lower_bound..self.weight_upper_bound))
            } else {
                None
            };

            graph.add_edge(a, b, weight)
        }

        if self.connected {
            'connection_test: loop {
                let mut manager = TraversalManager::new(GraphTraversers::SimpleBreadth);

                manager.new_traversal(NodeIndex(0), NodeIndex(self.node_count as usize - 1));

                while manager.currently_traversing {
                    manager.update(&mut graph)
                }

                let mut edges_to_add = Vec::new();

                if let Some(traversal) = manager.traversal {
                    if traversal.visited.len() != self.node_count as usize {
                        for node in graph.get_nodes_mut() {
                            if !traversal.visited.contains(&node.get_id()) {
                                let id = node.get_id();

                                let mut b = rng.gen_range(0..node_count) as usize;

                                while id.0 == b && node_count > 1 {
                                    b = rng.gen_range(0..node_count) as usize;
                                }

                                let mut b = NodeIndex(b);

                                while node.get_edges().iter().any(|e| {
                                    let (c, d) = e.get_nodes();
                                    if c == id {
                                        d == b
                                    } else if !self.directed {
                                        c == b
                                    } else {
                                        false
                                    }
                                }) {
                                    b = NodeIndex(rng.gen_range(0..node_count) as usize);
                                }

                                let weight = if self.weights {
                                    Some(rng.gen_range(
                                        self.weight_lower_bound..self.weight_upper_bound,
                                    ))
                                } else {
                                    None
                                };

                                edges_to_add.push((id, b, weight));
                            }
                        }

                        graph.reset();

                        for (a, b, weight) in edges_to_add.into_iter() {
                            graph.add_edge(a, b, weight);

                            let mut manager = TraversalManager::new(GraphTraversers::SimpleBreadth);
                            manager.new_traversal(
                                NodeIndex(0),
                                NodeIndex(self.node_count as usize - 1),
                            );

                            while manager.currently_traversing {
                                manager.update(&mut graph)
                            }

                            if traversal.visited.len() == self.node_count as usize {
                                break 'connection_test;
                            }

                            graph.reset();
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        graph.reset();
        graph
    }
}

impl Default for RandomGraphMenu {
    fn default() -> Self {
        RandomGraphMenu {
            graph_name: String::new(),
            connected: false,
            directed: false,
            node_count: 3,
            edge_count: 4,
            weights: false,
            weight_lower_bound: 1.0,
            weight_upper_bound: 5.0,
        }
    }
}
