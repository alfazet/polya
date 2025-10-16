use eframe::egui;
use egui::{Pos2, Vec2};

use crate::{
    drawing::LineAlgorithm,
    state::{CreatingState, EditingState, EmptyState},
};

mod constants;
mod drawing;
mod edge;
mod geometry;
mod polygon;
mod polyline;
mod state;
mod vertex;

enum AppState {
    // nothing is on the screen
    Empty(EmptyState),
    // we're creating the polygon,
    // the user can only edit the polyline
    Creating(CreatingState),
    // the polygon is on the screen already,
    // and the user can edit it
    Editing(EditingState),
}

struct PolygonEditor {
    app_state: AppState,
    origin_offset: Vec2,
    line_algo: LineAlgorithm,
    popup_open: bool,
}

impl PolygonEditor {
    fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self {
            app_state: AppState::Empty(EmptyState::default()),
            origin_offset: Vec2::ZERO,
            line_algo: LineAlgorithm::Builtin,
            popup_open: false,
        }
    }

    fn handle_key(&mut self, key: egui::Key) {
        match key {
            egui::Key::Delete => self.app_state = AppState::Empty(EmptyState::default()),
            egui::Key::ArrowLeft => self.origin_offset += constants::STEP * Vec2::LEFT,
            egui::Key::ArrowRight => self.origin_offset += constants::STEP * Vec2::RIGHT,
            egui::Key::ArrowUp => self.origin_offset += constants::STEP * Vec2::UP,
            egui::Key::ArrowDown => self.origin_offset += constants::STEP * Vec2::DOWN,
            egui::Key::Num0 => self.origin_offset = Vec2::ZERO,
            egui::Key::A => {
                self.line_algo = match self.line_algo {
                    LineAlgorithm::Builtin => LineAlgorithm::Bresenham,
                    LineAlgorithm::Bresenham => LineAlgorithm::Builtin,
                }
            }
            _ => (),
        }
    }
}

impl eframe::App for PolygonEditor {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        ctx.set_zoom_factor(2.0);
        // handling events
        let events = ctx.input(|input| input.events.clone());
        let mut pressed_key = None;
        for event in events {
            if let egui::Event::Key { key, pressed, .. } = event
                && !pressed
            {
                pressed_key = Some(key);
                break;
            }
        }
        let pointer = ctx.input(|input| input.pointer.clone());
        let pointer_pos = pointer.interact_pos().map(|pos| pos - self.origin_offset);
        match &mut self.app_state {
            AppState::Empty(state) => {
                if let Some(pos) = pointer_pos
                    && pointer.primary_released()
                {
                    self.app_state = AppState::Creating(CreatingState::new(pos));
                }
                if let Some(key) = pressed_key {
                    self.handle_key(key);
                }
            }
            AppState::Creating(state) => {
                if let Some(pos) = pointer_pos
                    && pointer.primary_released()
                {
                    if state.is_closing_click(pos) {
                        // close the polyline and start the editing
                        self.app_state = AppState::Editing(EditingState::new(state));
                    } else {
                        state.append_vertex(pos);
                    }
                }
                if let Some(key) = pressed_key {
                    self.handle_key(key);
                }
            }
            AppState::Editing(state) => {
                if let Some(pos) = pointer_pos {
                    if pointer.primary_released() {
                        state.check_click(pos);
                    } else if pointer.is_decidedly_dragging() {
                        state.drag_vertex(pointer.delta());
                    }
                }
                if let Some(key) = pressed_key {
                    match key {
                        egui::Key::X => state.remove_vertex(),
                        egui::Key::S => state.subdivide_edge(),
                        egui::Key::V => state.toggle_vertical(),
                        egui::Key::D => state.toggle_diagonal(),
                        egui::Key::L => state.length_dialog(),
                        egui::Key::B => state.toggle_bezier(),
                        _ => self.handle_key(key),
                    }
                }
            }
        }
        // drawing
        match &mut self.app_state {
            AppState::Empty(state) => state.draw(ctx),
            AppState::Creating(state) => state.draw(ctx, self.origin_offset, self.line_algo),
            AppState::Editing(state) => {
                // apply constraints
                state.apply_constraints();
                state.draw(ctx, self.origin_offset, self.line_algo);
            }
        }
    }
}

fn main() {
    simple_logging::log_to_stderr(log::LevelFilter::Warn);
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Polygon Editor",
        native_options,
        Box::new(|cc| Ok(Box::new(PolygonEditor::new(cc)))),
    );
}
