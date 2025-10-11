use eframe::egui;
use egui::Pos2;

use crate::state::{CreatingState, EditingState};

mod edge;
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
        let keys = ctx.input(|input| input.keys_down.clone());
        for key in keys {
            match key {
                egui::Key::Delete => self.run_empty_state(),
                _ => break,
            }
        }
        let pointer = ctx.input(|input| input.pointer.clone());
        if let Some(clicked_pos) = pointer.interact_pos()
            && pointer.primary_released()
        {
            match &mut self.app_state {
                AppState::Empty => {
                    self.run_creating_state(clicked_pos);
                }
                AppState::Creating(state) => {
                    if state.is_closing_click(clicked_pos) {
                        state.close();
                        self.run_editing_state();
                    } else {
                        state.append_vertex(clicked_pos);
                    }
                }
                AppState::Editing(_) => (), // check which edge/vertex was clicked and go from there
            }
        }

        // TODO: draw the top bar
        match &self.app_state {
            AppState::Empty => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Click anywhere to start drawing.");
                });
            }
            AppState::Creating(state) => state.draw(ctx),
            AppState::Editing(state) => state.draw(ctx),
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
