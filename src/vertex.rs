use egui::Pos2;

#[derive(Clone, Copy, Default)]
pub enum VertexKind {
    G0,
    G1,
    #[default]
    C1,
}

#[derive(Clone, Copy, Default)]
pub enum Constraint {
    #[default]
    None,
    Length(f32),
    Vertical,
    Diagonal,
    Bezier(Pos2, Pos2),
    CircleArc(Pos2, f32),
}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub pos: Pos2,
    pub kind: VertexKind,
    pub constraint: Constraint,
}

impl Vertex {
    pub fn new(pos: Pos2) -> Self {
        Self {
            pos,
            kind: VertexKind::default(),
            constraint: Constraint::default(),
        }
    }
}
