use egui::Pos2;

#[derive(Clone, Copy, Default)]
pub enum VertexKind {
    G0,
    G1,
    #[default]
    C1,
}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub pos: Pos2,
    pub kind: VertexKind,
}

impl Vertex {
    pub fn new(pos: Pos2) -> Self {
        Self {
            pos,
            kind: VertexKind::default(),
        }
    }
}
