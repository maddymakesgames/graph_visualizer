use std::collections::HashMap;

use egui::{ComboBox, DragValue, Key, TextEdit, Ui, Widget};

use crate::{
    app::{GraphApp, INTERNAL_HEIGHT, INTERNAL_WIDTH},
    graph::Graph,
    menus::Menu,
};

#[derive(Default)]
pub struct NodesMenu {
    curr_adding_node_text: String,
    curr_editing_node: usize,
    node_data: Vec<NodeMenuData>,
}

impl Menu for NodesMenu {
    fn ui(&mut self, app: &mut GraphApp, ui: &mut Ui) {
        if let Some(graph) = app.get_curr_graph() {
            ui.horizontal(|ui| {
                ui.label("Add Node");

                let node_name_editor = TextEdit::singleline(&mut self.curr_adding_node_text)
                    .hint_text("Node Name")
                    .desired_width(100.0)
                    .show(ui);

                if (node_name_editor.response.lost_focus() || ui.button("X").clicked())
                    && !self.curr_adding_node_text.is_empty()
                {
                    graph.add_node(
                        (500., 500.),
                        std::mem::take(&mut self.curr_adding_node_text),
                        Vec::new(),
                    );

                    self.node_data.push(NodeMenuData {
                        adding_index: 0,
                        removal_index: 0,
                        weight: if graph.is_weighted() { Some(1.0) } else { None },
                    });

                    if ui.input().key_pressed(Key::Enter) {
                        node_name_editor.response.request_focus()
                    }
                }
            });

            let index_name_map = graph
                .get_nodes()
                .iter()
                .map(|n| (n.get_id(), n.get_name().to_owned()))
                .collect::<HashMap<_, _>>();
            let usize_to_idx = graph
                .get_nodes()
                .iter()
                .map(|n| n.get_id())
                .collect::<Vec<_>>();

            let name_list = usize_to_idx
                .iter()
                .map(|idx| graph.get_node(*idx).get_name().to_owned())
                .collect::<Vec<_>>();

            let mut edge_to_add = None;
            let mut edge_to_remove = None;

            ComboBox::from_label("Nodes").show_index(
                ui,
                &mut self.curr_editing_node,
                index_name_map.len() + 1,
                |i| {
                    if i == 0 {
                        "Select Node".to_owned()
                    } else {
                        name_list[i - 1].to_owned()
                    }
                },
            );

            if self.curr_editing_node == 0 {
                return;
            }

            if let Some(node) = graph.try_get_node_mut(usize_to_idx[self.curr_editing_node - 1]) {
                let (x, y) = node.get_pos_mut();

                DragValue::new(x)
                    .clamp_range(0.0..=INTERNAL_WIDTH)
                    .speed(1.0)
                    .prefix("X: ")
                    .ui(ui);
                DragValue::new(y)
                    .clamp_range(0.0..=INTERNAL_HEIGHT)
                    .speed(1.0)
                    .prefix("Y: ")
                    .ui(ui);

                let data = &mut self.node_data[usize_to_idx
                    .iter()
                    .position(|i| *i == node.get_id())
                    .unwrap()];

                let edges = node.get_edges();

                ui.horizontal(|ui| {
                    ComboBox::from_label("Remove Edge").show_index(
                        ui,
                        &mut data.removal_index,
                        edges.len() + 1,
                        |i| {
                            if i == 0 {
                                "Select Node".to_owned()
                            } else {
                                let nodes = edges[i - 1].get_nodes();
                                if nodes.0 == node.get_id() {
                                    index_name_map[&nodes.1].clone()
                                } else {
                                    index_name_map[&nodes.0].clone()
                                }
                            }
                        },
                    );

                    if ui.button("X").clicked() && data.removal_index > 0 {
                        edge_to_remove = Some(edges[data.removal_index - 1]);
                        data.removal_index -= 1;
                    }
                });

                ui.horizontal(|ui| {
                    ComboBox::from_label("Add Connection").show_index(
                        ui,
                        &mut data.adding_index,
                        index_name_map.len(),
                        |i| name_list[i].to_owned(),
                    );

                    if let Some(weight) = data.weight.as_mut() {
                        ui.horizontal(|ui| {
                            ui.label("Weight");
                            DragValue::new(weight).ui(ui);
                        });
                    }

                    if ui.button("X").clicked() {
                        let idx = usize_to_idx[data.adding_index];
                        edge_to_add = Some((node.get_id(), idx));
                    }
                });

                if let Some((a, b)) = edge_to_add {
                    graph.add_edge(a, b, data.weight)
                }

                if let Some(e) = edge_to_remove {
                    graph.remove_edge(e)
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        "Nodes"
    }

    fn graph_updated(&mut self, graph: &Graph) {
        self.curr_editing_node = 0;
        self.node_data = vec![NodeMenuData::default(); graph.get_nodes().len()];
    }
}

#[derive(Default, Clone, Copy)]
pub struct NodeMenuData {
    adding_index: usize,
    removal_index: usize,
    weight: Option<f32>,
}
