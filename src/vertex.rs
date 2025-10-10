use egui::Pos2;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub pos: Pos2,
    pub id: u32,
}

impl Vertex {
    pub fn new(pos: Pos2, id: u32) -> Self {
        Self { pos, id }
    }
}
