use egui::Pos2;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub pos: Pos2,
}

impl Vertex {
    pub fn new(pos: Pos2) -> Self {
        Self { pos }
    }
}
