use egui::Pos2;

use crate::{
    edge::{Edge, EdgeKind},
    vertex::Vertex,
};

pub struct Polyline {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
}

impl Default for Polyline {
    fn default() -> Self {
        // Self {
        //     vertices: Vec::new(),
        //     edges: Vec::new(),
        // }
        let vertices = vec![
            Vertex::new(Pos2::new(100.0, 100.0), 1),
            Vertex::new(Pos2::new(200.0, 100.0), 2),
            Vertex::new(Pos2::new(200.0, 200.0), 3),
            Vertex::new(Pos2::new(100.0, 200.0), 4),
        ];
        let edges = vec![
            Edge::new(vertices[0], vertices[1], 1, EdgeKind::Straight),
            Edge::new(vertices[1], vertices[2], 2, EdgeKind::Straight),
            Edge::new(vertices[2], vertices[3], 3, EdgeKind::Straight),
            Edge::new(vertices[3], vertices[0], 4, EdgeKind::Straight),
        ];
        Self { vertices, edges }
    }
}

impl Polyline {
    pub fn len(&self) -> usize {
        self.edges.len()
    }

    pub fn is_closed(&self) -> bool {
        match self.vertices.last() {
            Some(last) => last.id == self.vertices[0].id,
            None => false,
        }
    }
}
