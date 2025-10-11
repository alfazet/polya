use egui::Pos2;

use crate::{
    edge::{Edge, EdgeKind},
    vertex::Vertex,
};

#[derive(Clone, Default)]
pub struct Polyline {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
}

impl Polyline {
    pub fn new(iniitial_pos: Pos2) -> Self {
        let vertices = vec![Vertex::new(iniitial_pos)];
        let edges = Vec::new();

        Self { vertices, edges }
    }

    pub fn append_vertex(&mut self, pos: Pos2) {
        let new_vertex = Vertex::new(pos);
        if let Some(last_vertex) = self.vertices.last() {
            self.edges.push(Edge::new(*last_vertex, new_vertex));
        }
        self.vertices.push(new_vertex);
    }

    pub fn close(&mut self) {
        self.edges.push(Edge::new(
            self.edges.last().unwrap().end,
            self.edges.first().unwrap().start,
        ));
    }
}
