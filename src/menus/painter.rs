use egui::{DragValue, Grid, Widget};

use crate::{graph::Graph, menus::Menu, painter::GraphPainter};

impl Menu for GraphPainter {
    type NeededData = ();

    fn ui(&mut self, _: &mut (), ui: &mut egui::Ui) {
        ui.heading("Node Settings");
        ui.separator();
        Grid::new("node settings").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Node Color");
                ui.color_edit_button_srgba(&mut self.node_color);
            });

            ui.horizontal(|ui| {
                ui.label("Start Node Color");
                ui.color_edit_button_srgba(&mut self.start_color);
            });

            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("End Node Color");
                ui.color_edit_button_srgba(&mut self.end_node_color);
            });

            ui.horizontal(|ui| {
                ui.label("Seen Node Color");
                ui.color_edit_button_srgba(&mut self.seen_color);
            });

            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("Visited Node Color");
                ui.color_edit_button_srgba(&mut self.visited_color);
            });

            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("Node Radius");
                DragValue::new(&mut self.node_radius).ui(ui);
            });
            ui.horizontal(|ui| {
                ui.label("Node Stroke");
                DragValue::new(&mut self.node_stroke).ui(ui);
            });

            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("Node Text Color");
                ui.color_edit_button_srgba(&mut self.node_text_color);
            });

            ui.horizontal(|ui| {
                ui.label("Node Text Size");
                DragValue::new(&mut self.node_text_size)
                    .clamp_range(1..=100)
                    .ui(ui);
            });
        });

        ui.heading("Edge Settings");
        ui.separator();
        Grid::new("edge settings").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Edge Color");
                ui.color_edit_button_srgba(&mut self.edge_color);
            });

            ui.horizontal(|ui| {
                ui.label("Edge Stroke");
                DragValue::new(&mut self.edge_stroke).ui(ui);
            });

            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("Path Color");
                ui.color_edit_button_srgba(&mut self.path_color);
            });

            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("Arrow Length");
                DragValue::new(&mut self.arrow_length)
                    .clamp_range(0..=100)
                    .ui(ui);
            });

            ui.horizontal(|ui| {
                ui.label("Curved Arrow Angle");
                DragValue::new(&mut self.curved_arrow_angle)
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
                ui.color_edit_button_srgba(&mut self.weight_text_color);
            });

            ui.horizontal(|ui| {
                ui.label("Weight Text Size");
                DragValue::new(&mut self.weight_text_size)
                    .clamp_range(1..=100)
                    .ui(ui);
            });

            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("Text Color Background");
                ui.color_edit_button_srgba(&mut self.text_background_color);
            });
        });
    }

    fn name(&self) -> &'static str {
        "Painter Settings"
    }

    fn graph_updated(&mut self, _graph_index: &Graph) {}
}
