use egui::Color32;

pub const ID_SIDEBAR_LEFT: &str = "sidebar_left";
pub const ID_VERTEX_CONTEXT_MENU: &str = "vertex_context_menu";
pub const ID_EDGE_CONTEXT_MENU: &str = "edge_context_menu";
pub const ID_FIXED_LEN_DIALOG: &str = "fixed_len_dialog";

pub const COLOR_BKG: Color32 = Color32::BLACK;
pub const COLOR_VERTEX_PRI: Color32 = Color32::WHITE;
pub const COLOR_VERTEX_SEC: Color32 = Color32::RED;
pub const COLOR_VERTEX_TER: Color32 = Color32::DARK_RED;
pub const COLOR_EDGE_PRI: Color32 = Color32::WHITE;
pub const COLOR_EDGE_SEC: Color32 = Color32::RED;
pub const COLOR_EDGE_LABEL: Color32 = Color32::LIGHT_RED;
pub const COLOR_VERTEX_LABEL: Color32 = Color32::LIGHT_RED;

pub const SIZE_STROKE: f32 = 1.0;
pub const SIZE_VERTEX: f32 = 4.0;
pub const SIZE_CONTROL_VERTEX: f32 = 6.0;
pub const SIZE_DASHES: f32 = 3.0;
pub const SIZE_GAPS: f32 = 5.0;
pub const SIZE_HITRADIUS: f32 = 2.0;
pub const SIZE_MARGIN: i8 = 15;
pub const SIZE_CONTEXT_MENU: f32 = 200.0;
pub const SIZE_CONTEXT_MENU_OFFSET: f32 = 10.0;
pub const SIZE_MIN_EDGE_LENGTH: f32 = 1.0;
pub const SIZE_MAX_EDGE_LENGTH: f32 = 1000.0;
pub const SIZE_LABEL_FONT: f32 = 14.0;
pub const SIZE_LABEL_OFFSET: f32 = 10.0;

pub const EPS: f32 = 0.01;
pub const DOT_EPS: f32 = 0.001;
pub const DIST_EPS: f32 = 0.1;
pub const BEZIER_DT: f32 = 0.001;
pub const ARC_DALPHA: f32 = 0.001;

pub const MAX_RESOLVING_ITERS: u8 = 64;
