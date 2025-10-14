use egui::Pos2;

use crate::{constants, geometry, vertex::Vertex};

#[derive(Clone, Copy)]
pub struct Edge {
    pub start: Pos2,
    pub end: Pos2,
}

impl Edge {
    pub fn new(start: Pos2, end: Pos2) -> Self {
        Self { start, end }
    }

    pub fn is_near(&self, pos: Pos2) -> bool {
        let points = geometry::bresenham_points(self.start, self.end);
        points
            .iter()
            .any(|p| p.distance_sq(pos) <= constants::TOLERANCE * constants::TOLERANCE)
    }
}
