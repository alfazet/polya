use eframe::egui;

pub const STEP: f32 = 4.0;
pub const TOLERANCE: f32 = 6.0;
pub const VERTEX_RADIUS: f32 = 5.0;
pub const STROKE_WIDTH: f32 = 1.0;

pub const EDGE_COLOR_BASE: egui::Color32 = egui::Color32::WHITE;
pub const EDGE_COLOR_SELECTED: egui::Color32 = egui::Color32::PURPLE;
pub const VERTEX_COLOR_BASE: egui::Color32 = egui::Color32::GRAY;
pub const VERTEX_COLOR_SELECTED: egui::Color32 = egui::Color32::PURPLE;
