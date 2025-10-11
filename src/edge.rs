use egui::Pos2;

use crate::{constants, geometry, vertex::Vertex};

#[derive(Clone, Copy, Default)]
pub enum EdgeKind {
    #[default]
    Straight,
    FixedLength(f32),
    Vertical,
    Diagonal,
    Bezier(Pos2, Pos2),
    CircleArc(Pos2, f32),
}

#[derive(Clone, Copy)]
pub struct Edge {
    pub start: Vertex,
    pub end: Vertex,
    pub kind: EdgeKind,
}

impl Edge {
    pub fn new(start: Vertex, end: Vertex) -> Self {
        Self {
            start,
            end,
            kind: EdgeKind::default(),
        }
    }

    pub fn is_near(&self, pos: Pos2) -> bool {
        let points = geometry::bresenham_points(self.start.pos, self.end.pos);
        points
            .iter()
            .any(|p| p.distance_sq(pos) <= constants::TOLERANCE * constants::TOLERANCE)
    }
}
