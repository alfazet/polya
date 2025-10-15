use egui::{Pos2, Vec2};

use crate::{
    edge::Edge,
    geometry,
    vertex::{Constraint, Vertex},
};

const CONSTRAINT_ITERS: usize = 3;

#[derive(Clone, Default)]
enum ConstraintCheckDirection {
    #[default]
    Forwards,
    Backwards,
}

#[derive(Clone, Default)]
pub struct Polyline {
    pub vertices: Vec<Vertex>,
    direction: ConstraintCheckDirection,
}

impl Polyline {
    pub fn new(initial_pos: Pos2) -> Self {
        let vertices = vec![Vertex::new(initial_pos)];

        Self {
            vertices,
            direction: ConstraintCheckDirection::default(),
        }
    }

    pub fn get_edge(&self, i: usize) -> (Pos2, Pos2) {
        if i == self.vertices.len() - 1 {
            (self.vertices[i].pos, self.vertices[0].pos)
        } else {
            (self.vertices[i].pos, self.vertices[i + 1].pos)
        }
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

    pub fn any_vertical_neighbor(&self, i: usize) -> bool {
        let next = if i == self.vertices.len() - 1 {
            0
        } else {
            i + 1
        };
        let prev = if i == 0 {
            self.vertices.len() - 1
        } else {
            i - 1
        };

        matches!(self.vertices[next].constraint, Constraint::Vertical)
            || matches!(self.vertices[prev].constraint, Constraint::Vertical)
    }

    fn next_vertex(&self, i: usize) -> usize {
        let n = self.vertices.len();
        match self.direction {
            ConstraintCheckDirection::Forwards => (i + 1) % n,
            ConstraintCheckDirection::Backwards => (i + n - 1) % n,
        }
    }

    fn apply_vertical(&mut self, i: usize) {
        let next = self.next_vertex(i);
        self.vertices[next].pos.x = self.vertices[i].pos.x;
    }

    fn apply_diagonal(&mut self, i: usize) {
        let next = self.next_vertex(i);
        let mut angle = ((self.vertices[next].pos.y - self.vertices[i].pos.y)
            .atan2(self.vertices[next].pos.x - self.vertices[i].pos.x)
            / std::f32::consts::PI
            * 180.0) as i32;
        if angle < 0 {
            angle += 360;
        }
        let adjusted_angle = (angle / 90) * 90 + 45; // to enforce an angle of 90*k + 45 degrees
        let adjusted_angle_rad = (adjusted_angle as f32) * std::f32::consts::PI / 180.0;

        let len = (self.vertices[next].pos).distance(self.vertices[i].pos);
        self.vertices[next].pos.x = self.vertices[i].pos.x + len * adjusted_angle_rad.cos();
        self.vertices[next].pos.y = self.vertices[i].pos.y + len * adjusted_angle_rad.sin();
    }

    fn apply_length(&mut self, i: usize, target_len: f32) {
        let next = self.next_vertex(i);
        let v = Vec2::new(
            self.vertices[next].pos.x - self.vertices[i].pos.x,
            self.vertices[next].pos.y - self.vertices[i].pos.y,
        );
        let scale = target_len / v.length();
        let scaled_v = scale * v;
        self.vertices[next].pos = self.vertices[i].pos + scaled_v;
    }

    // try to apply all constraints, with the start-th vertex fixed
    pub fn apply_constraints(&mut self, start: usize) {
        let backup = self.vertices.clone();
        let mut any_ok = false;
        self.direction = ConstraintCheckDirection::default();

        for i in 0..CONSTRAINT_ITERS {
            let mut i = start;
            let ok = loop {
                match self.vertices[i].constraint {
                    Constraint::Vertical => self.apply_vertical(i),
                    Constraint::Diagonal => self.apply_diagonal(i),
                    Constraint::Length(len) => self.apply_length(i, len),
                    _ => (),
                }
                i = (i + 1) % self.vertices.len();
                if i == start {
                    break self.check_constraints();
                }
            };
            if ok {
                any_ok = true;
                break;
            }
            self.direction = match self.direction {
                ConstraintCheckDirection::Forwards => ConstraintCheckDirection::Backwards,
                ConstraintCheckDirection::Backwards => ConstraintCheckDirection::Forwards,
            };
        }

        if !any_ok {
            self.vertices = backup;
        }
    }

    // check if all constraints are satisfied with
    // the given vertex positions
    pub fn check_constraints(&self) -> bool {
        // TODO
        true
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

    pub fn toggle_vertical(&mut self, i: usize) {
        self.vertices[i].constraint = Constraint::Vertical;
    }

    pub fn toggle_diagonal(&mut self, i: usize) {
        self.vertices[i].constraint = Constraint::Diagonal;
    }

    pub fn toggle_length(&mut self, i: usize, length: f32) {
        self.vertices[i].constraint = Constraint::Length(length);
    }
}
