use egui::{DragValue, Grid, Widget};

use crate::{app::GraphApp, graph::Graph, menus::Menu};

#[derive(Default)]
pub struct GraphPainterMenu;

impl Menu for GraphPainterMenu {
    fn ui(&mut self, app: &mut GraphApp, ui: &mut egui::Ui) {
        let painter = app.get_graph_painter();
        ui.heading("Node Settings");
        ui.separator();
        Grid::new("node settings").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Node Color");
                ui.color_edit_button_srgba(&mut painter.node_color);
            });

            ui.horizontal(|ui| {
                ui.label("Start Node Color");
                ui.color_edit_button_srgba(&mut painter.start_color);
            });

            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("End Node Color");
                ui.color_edit_button_srgba(&mut painter.end_node_color);
            });

            ui.horizontal(|ui| {
                ui.label("Seen Node Color");
                ui.color_edit_button_srgba(&mut painter.seen_color);
            });

            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("Visited Node Color");
                ui.color_edit_button_srgba(&mut painter.visited_color);
            });

            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("Node Radius");
                DragValue::new(&mut painter.node_radius).ui(ui);
            });
            ui.horizontal(|ui| {
                ui.label("Node Stroke");
                DragValue::new(&mut painter.node_stroke).ui(ui);
            });

            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("Node Text Color");
                ui.color_edit_button_srgba(&mut painter.node_text_color);
            });

            ui.horizontal(|ui| {
                ui.label("Node Text Size");
                DragValue::new(&mut painter.node_text_size)
                    .clamp_range(1..=100)
                    .ui(ui);
            });
        });

        ui.heading("Edge Settings");
        ui.separator();
        Grid::new("edge settings").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Edge Color");
                ui.color_edit_button_srgba(&mut painter.edge_color);
            });

            ui.horizontal(|ui| {
                ui.label("Edge Stroke");
                DragValue::new(&mut painter.edge_stroke).ui(ui);
            });

            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("Path Color");
                ui.color_edit_button_srgba(&mut painter.path_color);
            });

            ui.end_row();

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

            ui.end_row();
        });

        ui.heading("Weight Settings");
        ui.separator();
        Grid::new("Weight settings").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Weight Text Color");
                ui.color_edit_button_srgba(&mut painter.weight_text_color);
            });

            ui.horizontal(|ui| {
                ui.label("Weight Text Size");
                DragValue::new(&mut painter.weight_text_size)
                    .clamp_range(1..=100)
                    .ui(ui);
            });

            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("Text Color Background");
                ui.color_edit_button_srgba(&mut painter.text_background_color);
            });
        });
    }

    fn name(&self) -> &'static str {
        "Painter Settings"
    }

    fn graph_updated(&mut self, _graph_index: &Graph) {}
}
