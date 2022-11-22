use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use eframe::CreationContext;
use egui::{
    Color32, ComboBox, Context, DragValue, Key, LayerId, TextEdit, Ui, Visuals, Widget, WidgetText,
    Window,
};
use rand::Rng;

use crate::{
    graph::{Graph, NodeIndex},
    painter::GraphPainter,
    traversers::{GraphTraversers, TraversalData},
};

pub const INTERNAL_WIDTH: f32 = 1000.0;
pub const INTERNAL_HEIGHT: f32 = 1000.0;

pub struct GraphApp {
    painter: GraphPainter,
    graphs: Vec<Graph>,
    menu_data: MenuData,
    curr_graph: usize,
    curr_drag: Option<NodeIndex>,
}

impl eframe::App for GraphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        Window::new("Graph Visualizer").show(ctx, |ui| {
            self.menu_data.draw(
                ui,
                &mut self.graphs,
                &mut self.curr_graph,
                &mut self.painter,
            )
        });

        let painter = ctx.layer_painter(LayerId::background());

        if let Some(graph) = self.graphs.get(self.curr_graph) {
            self.painter.paint_graph(graph, &painter);

            if let Some(traversal) = self.get_current_traversal() {
                if !traversal.currently_traversing {
                    if let Some(traversal) = &traversal.traversal {
                        self.painter.paint_path(traversal.end_node, graph, &painter);
                    }
                }
            }
        }

        self.handle_drag(ctx);

        self.update_traversal();
    }
}

impl GraphApp {
    pub fn new(cc: &CreationContext) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());
        Self::default()
    }

    fn handle_drag(&mut self, ctx: &Context) {
        match &mut self.graphs.get_mut(self.curr_graph) {
            Some(g) => {
                Self::handle_drag_internal(g, ctx, self.painter.node_radius, &mut self.curr_drag)
            }
            None => {}
        }
    }

    fn handle_drag_internal(
        graph: &mut Graph,
        ctx: &Context,
        node_radius: f32,
        curr_drag: &mut Option<NodeIndex>,
    ) {
        // This has to be done before we take input because ctx.input() locks all ctx
        let window_size = ctx.available_rect().size();
        let sf_x = window_size.x / INTERNAL_WIDTH;
        let sf_y = window_size.y / INTERNAL_HEIGHT;

        let is_over_menu = ctx.is_pointer_over_area();

        let input = ctx.input();
        let pointer = &input.pointer;

        if let Some(idx) = curr_drag {
            if pointer.any_released() {
                *curr_drag = None;
            } else if let Some(pos) = pointer.interact_pos() {
                let x = pos.x / sf_x;
                let y = pos.y / sf_y;

                *graph.get_node_mut(*idx).get_pos_mut() = (x, y);
            }
        } else if pointer.any_pressed() && !is_over_menu {
            if let Some(pos) = pointer.interact_pos() {
                for node in graph.get_nodes() {
                    let (x, y) = node.get_pos();
                    let scaled_x = x * sf_x;
                    let scaled_y = y * sf_y;

                    let offset_x = scaled_x - pos.x;
                    let offset_y = scaled_y - pos.y;

                    if offset_x * offset_x + offset_y * offset_y <= node_radius * node_radius {
                        *curr_drag = Some(node.get_id());
                        break;
                    }
                }
            }
        }
    }

    fn update_traversal(&mut self) {
        if let Some(traversal_data) = self.menu_data.traversal_data.get_mut(self.curr_graph) {
            if let Some(traverser) = &mut traversal_data.traversal {
                if traversal_data.auto && traversal_data.currently_traversing {
                    let now = Instant::now();
                    if let Some(dur) = now.checked_duration_since(traversal_data.last_traversal) {
                        if dur.as_millis() as u32 >= traversal_data.speed {
                            if match self.graphs.get_mut(self.curr_graph) {
                                Some(g) => traverser.step(g),
                                None => false,
                            } {
                                traversal_data.currently_traversing = false;
                            }
                            traversal_data.last_traversal = now;
                        }
                    }
                }
            }
        }
    }

    fn get_current_traversal(&self) -> Option<&TraversalMenu> {
        self.menu_data.traversal_data.get(self.curr_graph)
    }
}

impl Default for GraphApp {
    fn default() -> Self {
        Self {
            graphs: Vec::new(),
            menu_data: MenuData {
                graph_data: GraphMenu {
                    graph_name: String::new(),
                    directed: false,
                    weighted: false,
                    random_menu_open: false,
                    random_node_count: 12,
                    random_connected: false,
                    random_edge_count: 15,
                    random_weights: false,
                    random_weight_lower_bound: 1.0,
                    random_weight_upper_bound: 10.0,
                },
                curr_menu: Menus::Graphs,
                node_data: Vec::new(),
                traversal_data: Vec::new(),
            },
            painter: GraphPainter {
                node_color: Color32::WHITE,
                edge_color: Color32::RED,
                text_color: Color32::WHITE,
                text_background_color: Color32::GRAY,
                seen_color: Color32::BROWN,
                visited_color: Color32::DARK_GREEN,
                end_node_color: Color32::DARK_BLUE,
                path_color: Color32::GOLD,
                node_radius: 32.0,
                node_stroke: 6.0,
                edge_stroke: 5.0,
                node_text_size: 18,
                weight_text_size: 18,
                arrow_length: 15.0,
                curved_arrow_angle: 0.5,
            },
            curr_graph: 0,
            curr_drag: None,
        }
    }
}

struct MenuData {
    curr_menu: Menus,
    node_data: Vec<NodesMenu>,
    traversal_data: Vec<TraversalMenu>,
    graph_data: GraphMenu,
}

impl MenuData {
    fn draw(
        &mut self,
        ui: &mut Ui,
        graphs: &mut Vec<Graph>,
        curr_graph: &mut usize,
        painter: &mut GraphPainter,
    ) {
        self.curr_menu.selectable_ui(ui);

        match self.curr_menu {
            Menus::Graphs => self.graph_data.draw(
                ui,
                graphs,
                curr_graph,
                &mut self.traversal_data,
                &mut self.node_data,
            ),
            Menus::Nodes => {
                if let Some(g) = graphs.get_mut(*curr_graph) {
                    if let Some(data) = self.node_data.get_mut(*curr_graph) {
                        data.draw(g, ui)
                    }
                }
            }
            Menus::PainterSettings => self.draw_painter_settings(painter, ui),
            Menus::Traversals => {
                if let Some(g) = graphs.get_mut(*curr_graph) {
                    if let Some(data) = self.traversal_data.get_mut(*curr_graph) {
                        data.draw(g, ui)
                    }
                }
            }
            Menus::UISettings => self.draw_ui_settings(ui),
        }
    }

    fn draw_painter_settings(&self, painter: &mut GraphPainter, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Node Color");
            ui.color_edit_button_srgba(&mut painter.node_color);
        });
        ui.horizontal(|ui| {
            ui.label("Edge Color");
            ui.color_edit_button_srgba(&mut painter.edge_color);
        });
        ui.horizontal(|ui| {
            ui.label("Text Color");
            ui.color_edit_button_srgba(&mut painter.text_color);
        });
        ui.horizontal(|ui| {
            ui.label("Text Color Background");
            ui.color_edit_button_srgba(&mut painter.text_background_color);
        });
        ui.horizontal(|ui| {
            ui.label("End Node Color");
            ui.color_edit_button_srgba(&mut painter.end_node_color);
        });
        ui.horizontal(|ui| {
            ui.label("Path Color");
            ui.color_edit_button_srgba(&mut painter.path_color);
        });
        ui.horizontal(|ui| {
            ui.label("Node Radius");
            DragValue::new(&mut painter.node_radius).ui(ui);
        });
        ui.horizontal(|ui| {
            ui.label("Node Stroke");
            DragValue::new(&mut painter.node_stroke).ui(ui);
        });
        ui.horizontal(|ui| {
            ui.label("Edge Stroke");
            DragValue::new(&mut painter.edge_stroke).ui(ui);
        });
        ui.horizontal(|ui| {
            ui.label("Seen Node");
            ui.color_edit_button_srgba(&mut painter.seen_color);
        });
        ui.horizontal(|ui| {
            ui.label("Visited Node");
            ui.color_edit_button_srgba(&mut painter.visited_color);
        });
        ui.horizontal(|ui| {
            ui.label("Node Text Size");
            DragValue::new(&mut painter.node_text_size)
                .clamp_range(1..=100)
                .ui(ui);
        });
        ui.horizontal(|ui| {
            ui.label("Weight Text Size");
            DragValue::new(&mut painter.weight_text_size)
                .clamp_range(1..=100)
                .ui(ui);
        });
        ui.horizontal(|ui| {
            ui.label("Arrow Length");
            DragValue::new(&mut painter.arrow_length)
                .clamp_range(0..=100)
                .ui(ui);
        });
        ui.horizontal(|ui| {
            ui.label("Curved Arrow Angle");
            DragValue::new(&mut painter.curved_arrow_angle)
                .clamp_range(0.0..=std::f32::consts::FRAC_PI_2)
                .speed(0.01)
                .ui(ui);
        });
    }

    fn draw_ui_settings(&self, ui: &mut Ui) {
        let mut style = (*ui.ctx().style()).clone();
        ui.heading("Font Sizes");
        for (text_style, font_id) in style.text_styles.iter_mut() {
            ui.horizontal(|ui| {
                ui.label(WidgetText::from(text_style.to_string()).text_style(text_style.clone()));
                ui.add(DragValue::new(&mut font_id.size).clamp_range(5.0..=50.0));
            });
        }
        ui.ctx().set_style(style);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Menus {
    UISettings,
    PainterSettings,
    Nodes,
    Graphs,
    Traversals,
}

impl Menus {
    pub fn name(&self) -> &'static str {
        match self {
            Menus::Graphs => "Graphs",
            Menus::UISettings => "UI Settings",
            Menus::PainterSettings => "Painter Settings",
            Menus::Traversals => "Traversal",
            Menus::Nodes => "Nodes",
        }
    }

    pub fn selectable_ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            for menu in Self::values() {
                ui.selectable_value(self, menu, menu.name());
            }
        });
    }

    const fn values() -> [Menus; 5] {
        [
            Menus::Graphs,
            Menus::Nodes,
            Menus::Traversals,
            Menus::PainterSettings,
            Menus::UISettings,
        ]
    }
}

#[derive(Default)]
pub struct NodesMenu {
    curr_adding_node_text: String,
    curr_editing_node: usize,
    node_data: Vec<NodeMenuData>,
}

impl NodesMenu {
    fn draw(&mut self, graph: &mut Graph, ui: &mut Ui) {
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

        let node_menu_data = &mut self.node_data;

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

            let data = &mut node_menu_data[usize_to_idx
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

#[derive(Default, Clone, Copy)]
pub struct NodeMenuData {
    adding_index: usize,
    removal_index: usize,
    weight: Option<f32>,
}

struct TraversalMenu {
    alg: GraphTraversers,
    traversal: Option<TraversalData>,
    start_node: usize,
    end_node: usize,
    last_traversal: Instant,
    speed: u32,
    auto: bool,
    debug_view: bool,
    currently_traversing: bool,
}

impl TraversalMenu {
    fn draw(&mut self, graph: &mut Graph, ui: &mut Ui) {
        let algs = GraphTraversers::values();
        let mut curr_alg = algs.iter().position(|a| *a == self.alg).unwrap();

        ComboBox::from_label("Traversal Algorithm").show_index(
            ui,
            &mut curr_alg,
            algs.len(),
            |i| algs[i].name().to_owned(),
        );

        self.alg = algs[curr_alg];

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

        ui.checkbox(&mut self.auto, "Automatically Traverse");

        if self.auto {
            ui.horizontal(|ui| {
                ui.label("Milliseconds between steps");
                DragValue::new(&mut self.speed).clamp_range(0..=20).ui(ui);
            });
        } else if self.currently_traversing && ui.button("Step Traversal").clicked() {
            if let Some(data) = &mut self.traversal {
                if data.step(graph) {
                    self.currently_traversing = false;
                }
            }
        }

        if self.traversal.is_some() {
            if ui.button("Stop Traversal").clicked() {
                self.traversal = None;
                graph.reset();
            }
        } else {
            // Isn't collapsible because the button call has side effects
            // You're not supposed to do this but /shrug
            #[allow(clippy::collapsible_else_if)]
            if ui.button("Start Traversal").clicked() && self.start_node > 0 && self.end_node > 0 {
                self.traversal = Some(TraversalData::new(
                    usize_to_idx[self.start_node - 1],
                    usize_to_idx[self.end_node - 1],
                    self.alg,
                ));
                self.currently_traversing = true;
            }
        }

        ui.checkbox(&mut self.debug_view, "Debug View");

        if self.debug_view {
            if let Some(traversal) = &self.traversal {
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

struct GraphMenu {
    graph_name: String,
    directed: bool,
    weighted: bool,
    random_menu_open: bool,
    random_node_count: u8,
    random_connected: bool,
    random_edge_count: u8,
    random_weights: bool,
    random_weight_lower_bound: f32,
    random_weight_upper_bound: f32,
}

impl GraphMenu {
    // We actually want a Vec instead of a slice because we need to be able to insert
    #[allow(clippy::ptr_arg)]
    fn draw(
        &mut self,
        ui: &mut Ui,
        graphs: &mut Vec<Graph>,
        curr_graph: &mut usize,
        traversal_data: &mut Vec<TraversalMenu>,
        node_data: &mut Vec<NodesMenu>,
    ) {
        let mut graph_selection = if graphs.is_empty() {
            0
        } else {
            *curr_graph + 1
        };
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
            *curr_graph = graph_selection - 1;
        }

        ui.horizontal(|ui| {
            ui.label("Graph Name");
            ui.text_edit_singleline(&mut self.graph_name);
        });

        ui.checkbox(&mut self.directed, "Directed Graph");
        ui.checkbox(&mut self.weighted, "Weighted Graph");

        if ui.button("Create Graph").clicked() {
            let name = std::mem::take(&mut self.graph_name);
            graphs.push(Graph::new(name, self.directed, self.weighted));
            traversal_data.push(TraversalMenu {
                traversal: None,
                start_node: 0,
                end_node: 0,
                alg: GraphTraversers::DepthFirst,
                last_traversal: Instant::now(),
                speed: 10,
                auto: true,
                debug_view: false,
                currently_traversing: false,
            });
            node_data.push(NodesMenu::default())
        }

        if ui.button("Create Random Graph").clicked() {
            self.random_menu_open = true;
        }

        if self.random_menu_open {
            if let Some(true) = Window::new("Random Graph Creation")
                .show(ui.ctx(), |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Node Count");
                        DragValue::new(&mut self.random_node_count)
                            .clamp_range(1..=30)
                            .ui(ui);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Edge Count");
                        DragValue::new(&mut self.random_edge_count)
                            .clamp_range(
                                if self.random_connected {
                                    self.random_node_count as u32 - 1
                                } else {
                                    0
                                }
                                    ..=self.random_node_count as u32
                                        * (self.random_node_count as u32 - 1),
                            )
                            .ui(ui);
                    });

                    ui.checkbox(&mut self.random_connected, "Connected Graph");
                    ui.checkbox(&mut self.random_weights, "Weighted Graph");

                    if self.random_weights {
                        DragValue::new(&mut self.random_weight_lower_bound).ui(ui);

                        if self.random_weight_upper_bound < self.random_weight_lower_bound {
                            self.random_weight_upper_bound = self.random_weight_lower_bound;
                        }

                        DragValue::new(&mut self.random_weight_upper_bound)
                            .clamp_range(self.random_weight_lower_bound..=f32::INFINITY)
                            .ui(ui);
                    }

                    ui.button("Create Graph").clicked()
                })
                .map(|a| {
                    let b = a.inner;
                    matches!(b, Some(true))
                })
            {
                let node_count = self.random_node_count;
                let edge_count = self.random_edge_count;

                let mut graph =
                    Graph::new(self.graph_name.clone(), self.directed, self.random_weights);
                let mut rng = rand::thread_rng();

                for i in 0..node_count {
                    let x = rng.gen_range(5.0..995.0);
                    let y = rng.gen_range(5.0..995.0);
                    graph.add_node((x, y), i.to_string(), Vec::new());
                }

                for i in 0..edge_count {
                    // If we are making a connected graph, we ensure that each node gets at least 1 edge
                    let a = if self.random_connected && i < node_count {
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

                    let weight = if self.random_weights {
                        Some(rng.gen_range(
                            self.random_weight_lower_bound..self.random_weight_upper_bound,
                        ))
                    } else {
                        None
                    };

                    graph.add_edge(a, b, weight)
                }

                if self.random_connected {
                    'connection_test: loop {
                        let mut traversal = TraversalData::new(
                            NodeIndex(0),
                            NodeIndex(self.random_node_count as usize - 1),
                            GraphTraversers::SimpleBreadth,
                        );

                        while !traversal.step(&mut graph) {}

                        let mut edges_to_add = Vec::new();

                        if traversal.visited.len() != self.random_node_count as usize {
                            println!("Found disconnected graph");
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

                                    let weight = if self.random_weights {
                                        Some(rng.gen_range(
                                            self.random_weight_lower_bound
                                                ..self.random_weight_upper_bound,
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

                                let mut traversal = TraversalData::new(
                                    NodeIndex(0),
                                    NodeIndex(self.random_node_count as usize - 1),
                                    GraphTraversers::SimpleBreadth,
                                );

                                while !traversal.step(&mut graph) {}

                                if traversal.visited.len() == self.random_node_count as usize {
                                    break 'connection_test;
                                }

                                graph.reset();
                            }
                        } else {
                            break;
                        }
                    }
                }
                graph.reset();

                graphs.push(graph);
                traversal_data.push(TraversalMenu {
                    traversal: None,
                    start_node: 0,
                    end_node: 0,
                    alg: GraphTraversers::DepthFirst,
                    last_traversal: Instant::now(),
                    speed: 10,
                    auto: true,
                    debug_view: false,
                    currently_traversing: false,
                });
                node_data.push(NodesMenu {
                    curr_adding_node_text: String::new(),
                    curr_editing_node: 0,
                    node_data: vec![NodeMenuData::default(); self.random_node_count as usize],
                });

                self.random_menu_open = false;
            }
        }
    }
}
