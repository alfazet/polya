use egui::{Pos2, Vec2};

use crate::constants;

#[derive(Clone, Copy, Debug)]
pub enum EdgeConstraint {
    Vertical,
    DiagonalUp,   // /
    DiagonalDown, // \
    FixedLength(f32),
}

#[derive(Clone, Copy, Debug, Default)]
pub enum VertexConstraint {
    G0,
    G1,
    #[default]
    C1,
}

#[derive(Clone, Copy, Debug)]
pub struct CubicBezier {
    pub control: [Pos2; 2],
}

#[derive(Clone, Copy, Debug)]
pub struct CircleArc;

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub p: Pos2,
    pub bezier: Option<CubicBezier>,
    pub arc: Option<CircleArc>,
    pub edge_c: Option<EdgeConstraint>,
    pub vertex_c: VertexConstraint,
}

impl CubicBezier {
    pub fn new(control: [Pos2; 2]) -> Self {
        Self { control }
    }

    pub fn nearby_control_vertex(&self, p: Pos2) -> Option<usize> {
        if self.control[0].distance_sq(p)
            <= constants::SIZE_CONTROL_VERTEX * constants::SIZE_CONTROL_VERTEX
        {
            return Some(0);
        }
        if self.control[1].distance_sq(p)
            <= constants::SIZE_CONTROL_VERTEX * constants::SIZE_CONTROL_VERTEX
        {
            return Some(1);
        }

        None
    }
}

impl From<(f32, f32)> for Vertex {
    fn from(pair: (f32, f32)) -> Self {
        Self {
            p: Pos2::from(pair),
            bezier: None,
            arc: None,
            edge_c: None,
            vertex_c: VertexConstraint::default(),
        }
    }
}

impl Vertex {
    pub fn new(p: Pos2) -> Self {
        Self {
            p,
            bezier: None,
            arc: None,
            edge_c: None,
            vertex_c: VertexConstraint::default(),
        }
    }

    pub fn is_near(&self, other: Pos2) -> bool {
        self.p.distance_sq(other) <= constants::SIZE_VERTEX * constants::SIZE_VERTEX
    }

    pub fn move_bezier_control_vertex(&mut self, which: usize, new_p: Pos2) {
        if let Some(bezier) = &mut self.bezier {
            bezier.control[which] = new_p;
        }
    }
}
