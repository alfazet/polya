use egui::Pos2;

use crate::{constants, geometry, vertex::Vertex};

#[derive(Clone, Copy)]
pub enum EdgeKind {
    Straight,
    Bezier(Pos2, Pos2),
}

#[derive(Clone, Copy)]
pub struct Edge {
    pub start: Pos2,
    pub end: Pos2,
    pub kind: EdgeKind,
}

impl Edge {
    pub fn new(start: Pos2, end: Pos2, kind: EdgeKind) -> Self {
        Self { start, end, kind }
    }

    pub fn is_near(&self, pos: Pos2) -> bool {
        let points = geometry::bresenham_points(self.start, self.end);
        points
            .iter()
            .any(|p| p.distance_sq(pos) <= constants::TOLERANCE * constants::TOLERANCE)
    }
}
