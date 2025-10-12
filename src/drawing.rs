use eframe::egui;
use egui::{Pos2, Vec2};

use crate::{constants, geometry, polyline::Polyline};

#[derive(Clone, Copy, Default)]
pub enum LineAlgorithm {
    #[default]
    Builtin,
    Bresenham,
}

pub fn draw_polyline(
    ui: &egui::Ui,
    polyline: &Polyline,
    offset: Vec2,
    closed: bool,
    selected_vertex_id: Option<usize>,
    selected_edge_id: Option<usize>,
    algo: LineAlgorithm,
) {
    for (i, edge) in polyline.get_edges(closed).iter().enumerate() {
        let color = match selected_edge_id {
            Some(id) if i == id => constants::EDGE_COLOR_SELECTED,
            _ => constants::EDGE_COLOR_BASE,
        };
        match algo {
            LineAlgorithm::Builtin => {
                ui.painter().line_segment(
                    [edge.start + offset, edge.end + offset],
                    egui::Stroke::new(constants::STROKE_WIDTH, color),
                );
            }
            LineAlgorithm::Bresenham => {
                let points = geometry::bresenham_points(edge.start + offset, edge.end + offset);
                for point in points {
                    ui.painter().rect_filled(
                        egui::Rect::from_min_max(point, point + Vec2::new(1.0, 1.0)),
                        0,
                        color,
                    );
                }
            }
        }
    }
    for (i, vertex) in polyline.vertices.iter().enumerate() {
        ui.painter().circle_filled(
            vertex.pos + offset,
            constants::VERTEX_RADIUS,
            match selected_vertex_id {
                Some(id) if i == id => constants::VERTEX_COLOR_SELECTED,
                _ => constants::VERTEX_COLOR_BASE,
            },
        );
    }
}
