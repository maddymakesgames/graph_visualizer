use std::time::Instant;

use eframe::CreationContext;
use egui::{Context, DragValue, LayerId, Ui, Visuals, WidgetText, Window};

use crate::{
    graph::{Graph, NodeIndex},
    menus::{
        graphs::GraphMenu, nodes::NodesMenu, painter::GraphPainterMenu, traversals::TraversalMenu,
        Menu,
    },
    painter::GraphPainter,
};

pub const INTERNAL_WIDTH: f32 = 1000.0;
pub const INTERNAL_HEIGHT: f32 = 1000.0;

#[derive(Default)]
pub struct AppManager {
    menus: MenuData,
    graph_app: GraphApp,
}

impl eframe::App for AppManager {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.graph_app.update(ctx, &mut self.menus);
    }
}

impl AppManager {
    pub fn new(cc: &CreationContext) -> Self {
        cc.egui_ctx.set_visuals(Visuals::dark());
        Self::default()
    }
}

#[derive(Default)]
pub struct GraphApp {
    painter: GraphPainter,
    graphs: Vec<Graph>,
    curr_graph: usize,
    curr_drag: Option<NodeIndex>,
}

impl GraphApp {
    fn update(&mut self, ctx: &Context, menus: &mut MenuData) {
        let curr_graph = self.curr_graph;
        let len = self.graphs.len();

        Window::new("Graph Visualizer").show(ctx, |ui| menus.draw(ui, self));

        let painter = ctx.layer_painter(LayerId::background());

        if let Some(graph) = self.graphs.get(self.curr_graph) {
            self.painter.paint_graph(graph, &painter);

            if !menus.traversal_data.currently_traversing {
                if let Some(traversal) = &menus.traversal_data.traversal {
                    self.painter.paint_path(traversal.end_node, graph, &painter);
                }
            } else {
                ctx.request_repaint();
            }
        }

        self.handle_drag(ctx);

        self.update_traversal(menus);

        if self.curr_graph != curr_graph || (self.curr_graph == 0 && len != self.graphs.len()) {
            menus.graph_updated(self);
        }
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

    fn update_traversal(&mut self, menus: &mut MenuData) {
        if let Some(traverser) = &mut menus.traversal_data.traversal {
            if menus.traversal_data.auto && menus.traversal_data.currently_traversing {
                let now = Instant::now();
                if let Some(dur) = now.checked_duration_since(menus.traversal_data.last_traversal) {
                    if dur.as_millis() as u32 >= menus.traversal_data.speed {
                        if match self.graphs.get_mut(self.curr_graph) {
                            Some(g) => traverser.step(g),
                            None => false,
                        } {
                            menus.traversal_data.currently_traversing = false;
                        }
                        menus.traversal_data.last_traversal = now;
                    }
                }
            }
        }
    }

    pub fn get_curr_graph(&mut self) -> Option<&mut Graph> {
        self.graphs.get_mut(self.curr_graph)
    }

    pub fn get_graphs(&mut self) -> &mut Vec<Graph> {
        &mut self.graphs
    }

    pub fn get_curr_graph_index(&self) -> usize {
        self.curr_graph
    }

    pub fn set_curr_graph_index(&mut self, index: usize) {
        self.curr_graph = index;
    }

    pub fn get_graph_painter(&mut self) -> &mut GraphPainter {
        &mut self.painter
    }
}

#[derive(Default)]
struct MenuData {
    curr_menu: Menus,
    node_menu: NodesMenu,
    traversal_data: TraversalMenu,
    graph_data: GraphMenu,
    painter_menu: GraphPainterMenu,
}

impl MenuData {
    fn draw(&mut self, ui: &mut Ui, app: &mut GraphApp) {
        self.curr_menu.selectable_ui(ui);

        match self.curr_menu {
            Menus::Graphs => self.graph_data.ui(app, ui),
            Menus::Nodes => {
                self.node_menu.ui(app, ui);
            }
            Menus::PainterSettings => self.painter_menu.ui(app, ui),
            Menus::Traversals => self.traversal_data.ui(app, ui),
            Menus::UISettings => self.draw_ui_settings(ui),
        }
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

    fn graph_updated(&mut self, arg: &mut GraphApp) {
        if let Some(graph) = arg.get_curr_graph() {
            self.graph_data.graph_updated(graph);
            self.node_menu.graph_updated(graph);
            self.traversal_data.graph_updated(graph);
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Menus {
    UISettings,
    PainterSettings,
    Nodes,
    #[default]
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
