use eframe::egui;
use egui::{Pos2, Vec2};

use crate::{
    drawing::LineAlgorithm,
    state::{CreatingState, EditingState},
};

mod constants;
mod drawing;
mod edge;
mod geometry;
mod polygon;
mod polyline;
mod state;
mod vertex;

#[derive(Default)]
enum AppState {
    // nothing is on the screen
    #[default]
    Empty,
    // we're creating the polygon,
    // the user can only edit the polyline
    Creating(CreatingState),
    // the polygon is on the screen already,
    // and the user can edit it
    Editing(EditingState),
}

#[derive(Default)]
struct PolygonEditor {
    app_state: AppState,
    origin_offset: Vec2,
    line_algo: LineAlgorithm,
}

impl PolygonEditor {
    fn new(_: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn run_empty_state(&mut self) {
        if !matches!(self.app_state, AppState::Empty) {
            self.app_state = AppState::Empty;
        }
    }

    fn run_creating_state(&mut self, initial_pos: Pos2) {
        if !matches!(self.app_state, AppState::Creating(_)) {
            self.app_state = AppState::Creating(CreatingState::new(initial_pos));
        }
    }

    fn run_editing_state(&mut self) {
        if let AppState::Creating(state) = &self.app_state {
            self.app_state = AppState::Editing(EditingState::new(state));
        }
    }
}

impl eframe::App for PolygonEditor {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        ctx.set_zoom_factor(2.0);
        // TODO: refactor, first match on state, later check keys/clicks inside a given state
        let keys = ctx.input(|input| input.keys_down.clone());
        for key in keys {
            match key {
                egui::Key::Delete => self.run_empty_state(),
                egui::Key::ArrowLeft => self.origin_offset += constants::STEP * Vec2::LEFT,
                egui::Key::ArrowRight => self.origin_offset += constants::STEP * Vec2::RIGHT,
                egui::Key::ArrowUp => self.origin_offset += constants::STEP * Vec2::UP,
                egui::Key::ArrowDown => self.origin_offset += constants::STEP * Vec2::DOWN,
                egui::Key::Num0 => self.origin_offset = Vec2::ZERO,
                egui::Key::B => {
                    self.line_algo = match self.line_algo {
                        LineAlgorithm::Builtin => LineAlgorithm::Bresenham,
                        LineAlgorithm::Bresenham => LineAlgorithm::Builtin,
                    }
                }
                _ => break,
            }
        }
        let pointer = ctx.input(|input| input.pointer.clone());
        if let Some(clicked_pos) = pointer.interact_pos()
            && pointer.is_decidedly_dragging()
        {
            if let AppState::Editing(state) = &mut self.app_state {
                state.drag_vertex(pointer.delta());
            }
        } else if let Some(clicked_pos) = pointer.interact_pos()
            && pointer.primary_released()
        {
            let actual_pos = clicked_pos - self.origin_offset;
            match &mut self.app_state {
                AppState::Empty => {
                    self.run_creating_state(actual_pos);
                }
                AppState::Creating(state) => {
                    if state.is_closing_click(actual_pos) {
                        self.run_editing_state();
                    } else {
                        state.append_vertex(actual_pos);
                    }
                }
                AppState::Editing(state) => {
                    state.check_click(actual_pos);
                }
            }
        }

        // TODO: draw the top bar
        match &self.app_state {
            AppState::Empty => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Click anywhere to start drawing");
                });
            }
            AppState::Creating(state) => state.draw(ctx, self.origin_offset, self.line_algo),
            AppState::Editing(state) => state.draw(ctx, self.origin_offset, self.line_algo),
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
