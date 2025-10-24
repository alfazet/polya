use egui::{Color32, Painter, Pos2, Rect, Shape, Stroke, Vec2};

use crate::{
    calc, constants,
    vertex::{EdgeConstraint, Vertex, VertexConstraint},
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum LineAlgorithm {
    #[default]
    Default,
    Bresenham,
}

trait Bresenham {
    fn bresenham_line_segment(&self, points: [Pos2; 2], stroke: Stroke);
}

impl Bresenham for Painter {
    fn bresenham_line_segment(&self, points: [Pos2; 2], stroke: Stroke) {
        for p in calc::bresenham_points(points[0], points[1]) {
            // drawing 1x1 rects is the closest we can get to manipulating
            // single pixels in egui
            self.rect_filled(
                Rect::from_min_max(p, p + Vec2::splat(stroke.width)),
                0.0,
                stroke.color,
            );
        }
    }
}

fn render_line_segment(
    painter: &Painter,
    points: [Pos2; 2],
    stroke: Stroke,
    line_algo: LineAlgorithm,
) {
    match line_algo {
        LineAlgorithm::Bresenham => {
            painter.bresenham_line_segment(points, stroke);
        }
        LineAlgorithm::Default => {
            painter.line_segment(points, stroke);
        }
    }
}

fn render_cubic_bezier(
    painter: &Painter,
    endpoints: [Pos2; 2],
    control: [Pos2; 2],
    stroke: Stroke,
) {
    let bezier_points =
        calc::cubic_bezier_points(endpoints[0], endpoints[1], control[0], control[1]);
    for pair in bezier_points.windows(2) {
        painter.line_segment([pair[0], pair[1]], stroke);
    }
}

// calculates (center, radius) based on v0 and v1's constraints and draws the corresponding arc
fn render_circular_arc(
    painter: &Painter,
    v0: Vertex,
    v1: Vertex,
    prev: Vertex,
    next: Vertex,
    stroke: Stroke,
) {
    let (s, r) = calc::circular_arc_data(v0, v1, prev, next);
    let arc_points = calc::arc_points(v0.p, v1.p, s, r);
    for pair in arc_points.windows(2) {
        painter.line_segment([pair[0], pair[1]], stroke);
    }
}

pub fn render_polyline_edges(painter: &Painter, vertices: &[Vertex], line_algo: LineAlgorithm) {
    for pair in vertices.windows(2) {
        let (v0, v1) = (pair[0], pair[1]);
        let color = constants::COLOR_EDGE_PRI;
        render_line_segment(
            painter,
            [v0.p, v1.p],
            Stroke::new(constants::SIZE_STROKE, color),
            line_algo,
        );
    }
}

pub fn render_polygon_edges(
    painter: &Painter,
    vertices: &[Vertex],
    selected_edge_i: Option<usize>,
    line_algo: LineAlgorithm,
) {
    for i in 0..vertices.len() {
        let color = match selected_edge_i {
            Some(s_i) if s_i == i => constants::COLOR_EDGE_SEC,
            _ => constants::COLOR_EDGE_PRI,
        };
        let next_i = (i + 1) % vertices.len();
        let (v0, v1) = (vertices[i], vertices[next_i]);
        let stroke = Stroke::new(constants::SIZE_STROKE, color);
        if let Some(bezier) = vertices[i].bezier {
            render_cubic_bezier(
                painter,
                [v0.p, v1.p],
                [bezier.control[0], bezier.control[1]],
                stroke,
            );
            painter.add(Shape::dashed_line(
                &[v0.p, v1.p],
                stroke,
                constants::SIZE_DASHES,
                constants::SIZE_GAPS,
            ));
            painter.add(Shape::dashed_line(
                &[v0.p, bezier.control[0]],
                stroke,
                constants::SIZE_DASHES,
                constants::SIZE_GAPS,
            ));
            painter.add(Shape::dashed_line(
                &[bezier.control[0], bezier.control[1]],
                stroke,
                constants::SIZE_DASHES,
                constants::SIZE_GAPS,
            ));
            painter.add(Shape::dashed_line(
                &[bezier.control[1], v1.p],
                stroke,
                constants::SIZE_DASHES,
                constants::SIZE_GAPS,
            ));
            for i in 0..=1 {
                painter.rect_filled(
                    Rect::from_center_size(
                        bezier.control[i],
                        Vec2::splat(constants::SIZE_CONTROL_VERTEX),
                    ),
                    0.0,
                    constants::COLOR_VERTEX_TER,
                );
            }
        } else if let Some(arc) = vertices[i].arc {
            let prev = vertices[(i + vertices.len() - 1) % vertices.len()];
            let next = vertices[(i + 1) % vertices.len()];
            render_circular_arc(painter, v0, v1, prev, next, stroke);
        } else {
            let label = match v0.edge_c {
                Some(EdgeConstraint::Vertical) => "||".to_string(),
                Some(EdgeConstraint::DiagonalUp) => "/".to_string(),
                Some(EdgeConstraint::DiagonalDown) => "\\".to_string(),
                Some(EdgeConstraint::FixedLength(len)) => (len.round()).to_string(),
                _ => String::new(),
            };
            render_line_segment(painter, [v0.p, v1.p], stroke, line_algo);
            painter.text(
                calc::midpoint(v0.p, v1.p),
                egui::Align2::CENTER_CENTER,
                label,
                egui::FontId::proportional(constants::SIZE_LABEL_FONT),
                constants::COLOR_EDGE_LABEL,
            );
        }
    }
}

pub fn render_vertices(
    painter: &Painter,
    vertices: &[Vertex],
    selected_i: Option<usize>,
    labels: bool,
) {
    for (i, v) in vertices.iter().enumerate() {
        let color = match selected_i {
            Some(s_i) if s_i == i => constants::COLOR_VERTEX_SEC,
            _ => constants::COLOR_VERTEX_PRI,
        };
        painter.circle_filled(v.p, constants::SIZE_VERTEX, color);
        let label = if v.bezier.is_some()
            || vertices[(i + vertices.len() - 1) % vertices.len()]
                .bezier
                .is_some()
            || v.arc.is_some()
            || vertices[(i + vertices.len() - 1) % vertices.len()]
                .arc
                .is_some()
        {
            match v.vertex_c {
                VertexConstraint::G0 => "G0".to_string(),
                VertexConstraint::G1 => "G1".to_string(),
                VertexConstraint::C1 => "C1".to_string(),
            }
        } else {
            "".to_string()
        };
        if labels {
            painter.text(
                v.p + Vec2::splat(constants::SIZE_LABEL_OFFSET),
                egui::Align2::CENTER_CENTER,
                label,
                egui::FontId::proportional(constants::SIZE_LABEL_FONT),
                constants::COLOR_VERTEX_LABEL,
            );
        }
    }
}
