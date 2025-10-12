use egui::{Pos2, Vec2};

use crate::{
    edge::{Edge, EdgeKind},
    geometry,
    vertex::Vertex,
};

#[derive(Clone, Default)]
pub struct Polyline {
    pub vertices: Vec<Vertex>,
}

impl Polyline {
    pub fn new(initial_pos: Pos2) -> Self {
        let vertices = vec![Vertex::new(initial_pos)];

        Self { vertices }
    }

    pub fn get_edges(&self, closed: bool) -> Vec<Edge> {
        let mut edges = Vec::new();
        if self.vertices.len() <= 1 {
            return edges;
        }
        for pair in self.vertices.windows(2) {
            edges.push(Edge::new(pair[0].pos, pair[1].pos));
        }
        if closed {
            edges.push(Edge::new(
                self.vertices[self.vertices.len() - 1].pos,
                self.vertices[0].pos,
            ));
        }

        edges
    }

    pub fn append_vertex(&mut self, pos: Pos2) {
        self.vertices.push(Vertex::new(pos));
    }

    pub fn drag_vertex(&mut self, i: usize, delta: Vec2) {
        self.vertices[i].pos += delta;
    }

    pub fn remove_vertex(&mut self, i: usize) {
        if self.vertices.len() > 3 {
            self.vertices.remove(i);
        }
    }

    pub fn subdivide_edge(&mut self, i: usize) {
        if i == self.vertices.len() - 1 {
            self.append_vertex(geometry::midpoint(
                self.vertices.last().unwrap().pos,
                self.vertices.first().unwrap().pos,
            ));
        } else {
            self.vertices.insert(
                i + 1,
                Vertex::new(geometry::midpoint(
                    self.vertices[i].pos,
                    self.vertices[i + 1].pos,
                )),
            );
        }
    }
}
