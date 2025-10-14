use eframe::egui;
use egui::{Pos2, Vec2};

use crate::{
    constants,
    drawing::{self, LineAlgorithm},
    polygon::Polygon,
    polyline::Polyline,
};

#[derive(Default)]
pub struct EmptyState;

#[derive(Default)]
pub struct CreatingState {
    polyline: Polyline,
    line_algo: LineAlgorithm,
}

pub struct EditingState {
    polygon: Polygon,
    line_algo: LineAlgorithm,
    selected_vertex_id: Option<usize>,
    selected_edge_id: Option<usize>,
    dragged_vertex_id: Option<usize>,
}

impl EmptyState {
    pub fn draw(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Place the first vertex [LMB]");
        });
    }
}

impl CreatingState {
    pub fn new(initial_pos: Pos2) -> Self {
        let polyline = Polyline::new(initial_pos);
        Self {
            polyline,
            line_algo: LineAlgorithm::default(),
        }
    }

    pub fn draw(&self, ctx: &egui::Context, offset: Vec2, line_algo: LineAlgorithm) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Place and connect the vertices (restart with [Del])");
            drawing::draw_polyline(ui, &self.polyline, offset, false, Some(0), None, line_algo);
        });
    }

    pub fn is_closing_click(&self, pos: Pos2) -> bool {
        self.polyline.vertices.len() >= 3
            && self.polyline.vertices[0].pos.distance_sq(pos)
                <= constants::VERTEX_RADIUS * constants::VERTEX_RADIUS
    }

    pub fn append_vertex(&mut self, pos: Pos2) {
        self.polyline.append_vertex(pos);
    }
}

impl EditingState {
    pub fn new(c_state: &CreatingState) -> Self {
        Self {
            polygon: Polygon::new(c_state.polyline.clone()),
            line_algo: LineAlgorithm::default(),
            selected_vertex_id: None,
            selected_edge_id: None,
            dragged_vertex_id: None,
        }
    }

    pub fn draw(&self, ctx: &egui::Context, offset: Vec2, line_algo: LineAlgorithm) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Select vertices/edges to edit");
            ui.small("[Del]\tRemove polygon");
            ui.small("[A]\t\tToggle line-drawing algorithm");
            ui.small("[X]\t\tRemove vertex (if possible)");
            ui.small("[S]\t\tSubdivide edge");
            ui.small("[B]\t\tToggle Bézier curve");
            ui.small("[C]\t\tToggle circular arc");
            ui.small("[V]\t\tVertical edge constraint");
            ui.small("[D]\t\tDiagonal edge constraint");
            ui.small("[L]\t\tLength constraint");
            ui.small("[1]\t\tG0 continuity in vertex");
            ui.small("[2]\t\tG1 continuity in vertex");
            ui.small("[3]\t\tC1 continuity in vertex");
            drawing::draw_polyline(
                ui,
                &self.polygon.polyline,
                offset,
                true,
                self.selected_vertex_id,
                self.selected_edge_id,
                line_algo,
            );
        });
    }

    pub fn check_click(&mut self, pos: Pos2) {
        if let Some(i) = self.polygon.polyline.vertices.iter().position(|vertex| {
            vertex.pos.distance_sq(pos) <= constants::VERTEX_RADIUS * constants::VERTEX_RADIUS
        }) {
            self.selected_vertex_id = Some(i);
            self.selected_edge_id = None;
            return;
        }
        if let Some(i) = self
            .polygon
            .polyline
            .get_edges(true)
            .iter()
            .position(|edge| edge.is_near(pos))
        {
            self.selected_edge_id = Some(i);
            self.selected_vertex_id = None;
            return;
        }

        self.selected_vertex_id = None;
        self.selected_edge_id = None;
    }

    pub fn apply_constraints(&mut self) {
        let start = self.selected_vertex_id.unwrap_or(0);
        self.polygon.polyline.apply_constraints(start);
    }

    pub fn drag_vertex(&mut self, delta: Vec2) {
        if let Some(i) = self.selected_vertex_id {
            self.polygon.polyline.drag_vertex(i, delta);
        }
    }

    pub fn remove_vertex(&mut self) {
        if let Some(i) = self.selected_vertex_id {
            self.polygon.polyline.remove_vertex(i);
        }
        self.selected_vertex_id = None;
        self.selected_edge_id = None;
    }

    pub fn subdivide_edge(&mut self) {
        if let Some(i) = self.selected_edge_id {
            self.polygon.polyline.subdivide_edge(i);
        }
        self.selected_edge_id = None;
    }

    pub fn toggle_vertical(&mut self) {
        if let Some(i) = self.selected_edge_id
            && !self.polygon.polyline.any_vertical_neighbor(i)
        {
            self.polygon.polyline.toggle_vertical(i);
        }
        self.selected_vertex_id = None;
    }

    pub fn toggle_diagonal(&mut self) {
        if let Some(i) = self.selected_edge_id {
            self.polygon.polyline.toggle_diagonal(i);
        }
        self.selected_vertex_id = None;
    }
}
