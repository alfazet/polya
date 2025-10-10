use crate::vertex::Vertex;

pub enum EdgeKind {
    Straight,
    Bezier,
    CircleArc,
}

pub struct Edge {
    pub start: Vertex,
    pub end: Vertex,
    pub id: u32,
    pub kind: EdgeKind,
}

impl Edge {
    pub fn new(start: Vertex, end: Vertex, id: u32, kind: EdgeKind) -> Self {
        Self {
            start,
            end,
            id,
            kind,
        }
    }
}
