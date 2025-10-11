use eframe::egui;
use egui::Pos2;

use crate::{polygon::Polygon, polyline::Polyline};

const VERTEX_RADIUS: f32 = 5.0;

#[derive(Default)]
pub struct CreatingState {
    polyline: Polyline,
}

pub struct EditingState {
    polygon: Polygon,
}

impl CreatingState {
    pub fn new(initial_pos: Pos2) -> Self {
        let polyline = Polyline::new(initial_pos);
        Self { polyline }
    }

    pub fn draw(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Connect the vertices (DEL to restart)");
            draw_polyline(ui, &self.polyline, true);
        });
    }

    pub fn is_closing_click(&self, pos: Pos2) -> bool {
        self.polyline.vertices.len() >= 3
            && self.polyline.vertices[0].pos.distance_sq(pos) <= VERTEX_RADIUS * VERTEX_RADIUS
    }

    // join the last vertex with the first one to close the polyline into a polygon
    pub fn close(&mut self) {
        self.polyline.close();
    }

    pub fn append_vertex(&mut self, pos: Pos2) {
        self.polyline.append_vertex(pos);
    }
}

impl EditingState {
    pub fn new(c_state: &CreatingState) -> Self {
        Self {
            polygon: Polygon::new(c_state.polyline.clone()),
        }
    }

    pub fn draw(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Edit the polygon (DEL to delete)");
            draw_polyline(ui, &self.polygon.polyline, false);
        });
    }
}

fn draw_polyline(ui: &egui::Ui, polyline: &Polyline, mark_first: bool) {
    for edge in &polyline.edges {
        ui.painter().line_segment(
            [edge.start.pos, edge.end.pos],
            egui::Stroke::new(1.0, egui::Color32::WHITE),
        );
    }
    for (i, vertex) in polyline.vertices.iter().enumerate() {
        ui.painter().circle_filled(
            vertex.pos,
            VERTEX_RADIUS,
            if mark_first && i == 0 {
                egui::Color32::RED
            } else {
                egui::Color32::GRAY
            },
        );
    }
}
