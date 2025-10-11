use crate::vertex::Vertex;

#[derive(Clone, Copy)]
pub enum EdgeKind {
    Straight,
    Vertical,
    Diagonal,
    Bezier,
    CircleArc,
}

#[derive(Clone, Copy)]
pub struct Edge {
    pub start: Vertex,
    pub end: Vertex,
    pub kind: EdgeKind,
}

impl Edge {
    pub fn new(start: Vertex, end: Vertex, kind: EdgeKind) -> Self {
        Self { start, end, kind }
    }
}
