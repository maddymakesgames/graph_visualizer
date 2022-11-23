use std::ops::Sub;

use eframe::epaint::QuadraticBezierShape;
use egui::{Align2, Color32, FontId, Painter, Pos2, Rgba, Rounding, Stroke, Vec2};

use crate::{
    app::{INTERNAL_HEIGHT, INTERNAL_WIDTH},
    graph::{Edge, Graph, Node, NodeIndex, NodeState},
};

pub struct GraphPainter {
    pub start_color: Color32,
    pub node_color: Color32,
    pub edge_color: Color32,
    pub text_color: Color32,
    pub seen_color: Color32,
    pub text_background_color: Color32,
    pub visited_color: Color32,
    pub end_node_color: Color32,
    pub path_color: Color32,
    pub node_radius: f32,
    pub node_stroke: f32,
    pub edge_stroke: f32,
    pub node_text_size: u8,
    pub weight_text_size: u8,
    pub arrow_length: f32,
    pub curved_arrow_angle: f32,
}

impl GraphPainter {
    pub fn paint_graph(&self, graph: &Graph, painter: &Painter) {
        let mut weights_to_render = Vec::new();

        for node in graph.get_nodes() {
            for edge in node.get_edges() {
                self.paint_graph_edge(
                    graph,
                    &edge,
                    painter,
                    graph.is_directed(),
                    Stroke::from((self.edge_stroke, self.edge_color)),
                    &mut weights_to_render,
                )
            }
        }

        for (pos, weight) in weights_to_render {
            let text = weight.to_string();

            let galley = painter.layout_no_wrap(
                text,
                FontId::monospace(self.weight_text_size as f32),
                self.text_color,
            );

            let text_rect = galley.rect;

            painter.rect_filled(
                text_rect.translate(
                    Vec2::new(pos.x, pos.y)
                        - Vec2::new(text_rect.width() / 2.0, text_rect.height() / 2.0),
                ),
                Rounding::none(),
                self.text_background_color,
            );

            painter.galley(
                pos - Vec2::new(text_rect.width() / 2.0, text_rect.height() / 2.0),
                galley,
            )
        }

        for node in graph.get_nodes() {
            self.paint_graph_node(node, painter)
        }
    }

    pub fn paint_path(&self, end_node: NodeIndex, graph: &Graph, painter: &Painter) {
        let mut curr_node = graph.get_node(end_node);
        let Some(mut next_node) = curr_node.get_last_node().map(|n| graph.get_node(n)) else {
            return;
        };

        while curr_node.get_id() != next_node.get_id() {
            self.paint_graph_edge(
                graph,
                &Edge::new(next_node.get_id(), curr_node.get_id(), None),
                painter,
                graph.is_directed(),
                Stroke::from((self.edge_stroke, self.path_color)),
                &mut Vec::new(),
            );

            curr_node = next_node;
            next_node = if let Some(node) = curr_node.get_last_node() {
                graph.get_node(node)
            } else {
                return;
            };
        }
    }

    fn paint_graph_node(&self, node: &Node, painter: &Painter) {
        let window_size = painter.ctx().available_rect().size();
        let sf_x = window_size.x / INTERNAL_WIDTH;
        let sf_y = window_size.y / INTERNAL_HEIGHT;

        let (nx, ny) = node.get_pos();

        let scaled_node_pos = (nx * sf_x, ny * sf_y);

        painter.circle(
            scaled_node_pos.into(),
            self.node_radius,
            match node.get_state() {
                NodeState::None => Rgba::BLACK,
                NodeState::Start => Rgba::from(self.start_color),
                NodeState::Seen => Rgba::from(self.seen_color),
                NodeState::Visited => Rgba::from(self.visited_color),
                NodeState::End => Rgba::from(self.end_node_color),
            },
            (self.node_stroke, self.node_color),
        );

        painter.text(
            scaled_node_pos.into(),
            Align2::CENTER_CENTER,
            node.get_name(),
            FontId::monospace(self.node_text_size as f32),
            self.text_color,
        );
    }

    fn paint_graph_edge(
        &self,
        graph: &Graph,
        edge: &Edge,
        painter: &Painter,
        is_directed: bool,
        line_stroke: Stroke,
        weights_to_render: &mut Vec<(Pos2, f32)>,
    ) {
        let window_size = painter.ctx().available_rect().size();
        let sf_x = window_size.x / 1000.0;
        let sf_y = window_size.y / 1000.0;

        let (weight, n1, n2) = edge.get_weighted_nodes();

        let n1 = graph.get_node(n1);
        let n2 = graph.get_node(n2);

        let (x1, y1) = n1.get_pos();
        let (x2, y2) = n2.get_pos();

        let dx = (x2 - x1) * sf_x;
        let dy = (y2 - y1) * sf_y;

        let theta = (dy / dx).atan();

        let (cos, sin) = if x1 > x2 {
            (-theta.cos(), -theta.sin())
        } else {
            (theta.cos(), theta.sin())
        };

        let r = self.node_radius + self.node_stroke / 2.0;

        let new_n1 = (x1 * sf_x + r * cos, y1 * sf_y + r * sin);

        let new_n2 = (x2 * sf_x - r * cos, y2 * sf_y - r * sin);

        if is_directed {
            if n2.get_edges().iter().any(|e| {
                let nodes = e.get_nodes();
                nodes.1 == n1.get_id()
            }) {
                self.curved_arrow(
                    painter,
                    Pos2::new(x1 * sf_x, y1 * sf_y),
                    Pos2::new(x2 * sf_x, y2 * sf_y),
                    weight,
                    graph.is_weighted(),
                    line_stroke,
                    weights_to_render,
                );
            } else {
                self.arrow(
                    painter,
                    new_n1.into(),
                    Vec2::from(new_n2).sub(new_n1.into()),
                    line_stroke,
                );

                if graph.is_weighted() {
                    let x_text_pos = (n1.get_pos().0 + n2.get_pos().0) / 2.0 * sf_x;
                    let y_text_pos = ((n1.get_pos().1 + n2.get_pos().1) / 2.0 + 10.0) * sf_y;

                    weights_to_render.push((Pos2::new(x_text_pos, y_text_pos), weight))
                }
            }
        } else {
            painter.line_segment([new_n1.into(), new_n2.into()], line_stroke);

            if graph.is_weighted() {
                let x_text_pos = (n1.get_pos().0 + n2.get_pos().0) / 2.0 * sf_x;
                let y_text_pos = ((n1.get_pos().1 + n2.get_pos().1) / 2.0 + 10.0) * sf_y;

                weights_to_render.push((Pos2::new(x_text_pos, y_text_pos), weight))
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn curved_arrow(
        &self,
        painter: &Painter,
        a: Pos2,
        b: Pos2,
        weight: f32,
        is_weighted: bool,
        line_stroke: Stroke,
        weights_to_render: &mut Vec<(Pos2, f32)>,
    ) {
        // Find midpoint
        let mid_point_x = (a.x + b.x) / 2.0;
        let mid_point_y = (a.y + b.y) / 2.0;
        let mid_length = f32::sqrt((mid_point_x - a.x).powi(2) + (mid_point_y - a.y).powi(2));

        // Find dx, dy
        let dx = b.x - a.x;
        let dy = b.y - a.y;

        // If dx < 0 this will be -1
        let is_negative_dx = dx / dx.abs();

        // Calculate the control point by adding the normal vector to the midpoint and scaling based on the angle
        let unit_normal = Vec2::new(-dy, dx).normalized();
        let control_point_dist = mid_length * f32::atan(self.curved_arrow_angle);
        let control_point = Pos2::new(mid_point_x, mid_point_y) + unit_normal * control_point_dist;

        // Find angle from the x-axis to place the start and end nodes
        let angle_from_origin = f32::atan(dy / dx);
        // if dx < 0 the end points will be on the wrong side of the circumference
        let (start_point_angle, end_point_angle) = (
            self.curved_arrow_angle + angle_from_origin,
            angle_from_origin - self.curved_arrow_angle,
        );

        let radius = self.node_radius + self.node_stroke / 2.0;

        // Find start and end points on the circumference of the nodes
        let start_point = Pos2::new(
            radius * f32::cos(start_point_angle) * is_negative_dx + a.x,
            radius * f32::sin(start_point_angle) * is_negative_dx + a.y,
        );
        let end_point = Pos2::new(
            radius * -f32::cos(end_point_angle) * is_negative_dx + b.x,
            radius * -f32::sin(end_point_angle) * is_negative_dx + b.y,
        );

        let curve = QuadraticBezierShape::from_points_stroke(
            [start_point, control_point, end_point],
            false,
            Color32::TRANSPARENT,
            line_stroke,
        );

        if is_weighted {
            weights_to_render.push((curve.sample(0.5), weight));
        }

        painter.add(curve);

        // We want to imitate drawing an arrow from the control point to the end point
        let control_end_x = end_point.x - control_point.x;
        let control_end_y = end_point.y - control_point.y;

        self.arrow_pointy_bits(
            painter,
            control_point,
            Vec2::new(control_end_x, control_end_y),
            line_stroke,
        );
    }

    // egui's arrow() has the tips grow in size based on magnitude.
    // we want them to be constant so we recreate their function here
    fn arrow(&self, painter: &Painter, origin: Pos2, vec: Vec2, stroke: Stroke) {
        use egui::emath::*;
        let rot = Rot2::from_angle(std::f32::consts::TAU / 10.0);
        let tip_length = self.arrow_length;
        let tip = origin + vec;
        let dir = vec.normalized();

        painter.line_segment([origin, tip], stroke);
        painter.line_segment([tip, tip - tip_length * (rot * dir)], stroke);
        painter.line_segment([tip, tip - tip_length * (rot.inverse() * dir)], stroke);
    }

    // Same as Self::arrow except we don't draw the line from the start point and end point
    fn arrow_pointy_bits(&self, painter: &Painter, origin: Pos2, vec: Vec2, stroke: Stroke) {
        use egui::emath::*;
        let rot = Rot2::from_angle(std::f32::consts::TAU / 10.0);
        let tip_length = self.arrow_length;
        let tip = origin + vec;
        let dir = vec.normalized();

        painter.line_segment([tip, tip - tip_length * (rot * dir)], stroke);
        painter.line_segment([tip, tip - tip_length * (rot.inverse() * dir)], stroke);
    }
}
