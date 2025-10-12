use eframe::egui;
use egui::{Pos2, Vec2};

use crate::{
    constants,
    drawing::{self, LineAlgorithm},
    polygon::Polygon,
    polyline::Polyline,
};

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
            ui.heading("Connect the vertices (DEL to restart)");
            drawing::draw_polyline(ui, &self.polyline, offset, false, Some(0), None, line_algo);
        });
    }

    pub fn is_closing_click(&self, pos: Pos2) -> bool {
        self.polyline.vertices.len() >= 3
            && self.polyline.vertices[0].pos.distance_sq(pos)
                <= constants::VERTEX_RADIUS * constants::VERTEX_RADIUS
    }

    // join the last vertex with the first one to close the polyline into a polygon
    // pub fn close(&mut self) {
    //     self.polyline.close();
    // }

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
            ui.heading("Edit the polygon (DEL to delete)");
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

    pub fn drag_vertex(&mut self, delta: Vec2) {
        if let Some(i) = self.selected_vertex_id {
            self.polygon.polyline.drag_vertex(i, delta);
        }
    }
}
