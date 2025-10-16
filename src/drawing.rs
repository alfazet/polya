use eframe::egui;
use egui::{Pos2, Vec2};

use crate::{constants, edge::EdgeKind, geometry, polyline::Polyline};

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
        match edge.kind {
            EdgeKind::Straight => match algo {
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
            },
            EdgeKind::Bezier(c0, c1) => {
                let points = cubic_bezier(
                    edge.start + offset,
                    edge.end + offset,
                    c0 + offset,
                    c1 + offset,
                );
                for pair in points.windows(2) {
                    ui.painter().line_segment(
                        [pair[0], pair[1]],
                        egui::Stroke::new(constants::STROKE_WIDTH, color),
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

// returns points approximating the curve between p0 and p1
// with control points c0 and c1
pub fn cubic_bezier(p0: Pos2, p1: Pos2, c0: Pos2, c1: Pos2) -> Vec<Pos2> {
    let (p0, p1, c0, c1) = (
        Vec2::from((p0.x, p0.y)),
        Vec2::from((p1.x, p1.y)),
        Vec2::from((c0.x, c0.y)),
        Vec2::from((c1.x, c1.y)),
    );
    let a3 = -1.0 * p0 + 3.0 * c0 - 3.0 * c1 + p1;
    let a2 = 3.0 * p0 - 6.0 * c0 + 3.0 * c1;
    let a1 = -3.0 * p0 + 3.0 * c0;
    let a0 = p0;
    let d = constants::BEZIER_D;

    let delta3 = 6.0 * d * d * d * a3;
    let mut delta2 = 6.0 * d * d * d * a3 + 2.0 * d * d * a2;
    let mut delta = d * d * d * a3 + d * d * a2 + d * a1;
    let mut p = a0;
    let mut points = vec![p.to_pos2()];
    let mut t = 0.0;
    while t <= 1.0 {
        p += delta;
        delta += delta2;
        delta2 += delta3;
        points.push(p.to_pos2());
        t += d;
    }

    points
}
