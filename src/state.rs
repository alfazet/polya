use eframe::egui;

use crate::{polygon::Polygon, polyline::Polyline};

pub struct CreatingState {
    polyline: Polyline,
}

pub struct IdleState {
    polygon: Polygon,
}

impl Default for CreatingState {
    fn default() -> Self {
        Self {
            polyline: Polyline::default(),
        }
    }
}

impl CreatingState {
    pub fn new(polyline: Polyline) -> Self {
        Self { polyline }
    }

    pub fn draw(&self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            for vertex in &self.polyline.vertices {
                ui.painter()
                    .circle_filled(vertex.pos, 10.0, egui::Color32::RED);
            }
            for edge in &self.polyline.edges {
                ui.painter().line_segment(
                    [edge.start.pos, edge.end.pos],
                    egui::Stroke::new(1.0, egui::Color32::BLUE),
                );
            }
        });
    }
}

impl IdleState {
    pub fn draw(&self, ctx: &egui::Context) {}
}
