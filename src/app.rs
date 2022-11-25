use eframe::CreationContext;
use egui::{Context, LayerId, SelectableLabel, Ui, Visuals, Window};

use crate::{
    graph::{Graph, NodeIndex},
    menus::{menus, Menu, MENU_COUNT},
    painter::GraphPainter,
    traversers::TraversalManager,
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
    pub painter: GraphPainter,
    pub graphs: Vec<Graph>,
    pub curr_graph: usize,
    pub curr_drag: Option<NodeIndex>,
    pub traversal_manager: TraversalManager,
}

impl GraphApp {
    fn update(&mut self, ctx: &Context, menus: &mut MenuData) {
        let curr_graph = self.curr_graph;
        let len = self.graphs.len();

        Window::new("Graph Visualizer").show(ctx, |ui| menus.draw(ui, self));

        let painter = ctx.layer_painter(LayerId::background());

        if let Some(graph) = self.graphs.get(self.curr_graph) {
            self.painter.paint_graph(graph, &painter);

            if !self.traversal_manager.currently_traversing {
                if let Some(traversal) = &self.traversal_manager.traversal {
                    self.painter.paint_path(traversal.end_node, graph, &painter);
                }
            } else {
                ctx.request_repaint();
            }
        }

        self.handle_drag(ctx);

        if let Some(graph) = self.graphs.get_mut(self.curr_graph) {
            if self.traversal_manager.auto {
                self.traversal_manager.update(graph);
            }
        }

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
}

struct MenuData {
    curr_menu: usize,
    menus: [Box<dyn Menu>; MENU_COUNT],
}

impl MenuData {
    fn draw(&mut self, ui: &mut Ui, app: &mut GraphApp) {
        self.selectable_ui(ui);

        self.menus[self.curr_menu].ui(app, ui);
    }

    pub fn selectable_ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            for (i, menu) in self.menus.iter().enumerate() {
                if ui
                    .add(SelectableLabel::new(self.curr_menu == i, menu.name()))
                    .clicked()
                {
                    self.curr_menu = i;
                }
            }
        });
    }

    fn graph_updated(&mut self, app: &mut GraphApp) {
        if let Some(graph) = app.graphs.get_mut(app.curr_graph) {
            for menu in &mut self.menus {
                menu.graph_updated(graph);
            }
        }
    }
}

impl Default for MenuData {
    fn default() -> Self {
        MenuData {
            curr_menu: 0,
            menus: menus(),
        }
    }
}
