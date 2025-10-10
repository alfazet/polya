use eframe::egui;

use crate::state::{CreatingState, IdleState};

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
    Idle(IdleState),
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

    fn run_creating_state(&mut self) {
        if !matches!(self.app_state, AppState::Creating(_)) {
            self.app_state = AppState::Creating(CreatingState::default());
        }
    }
}

impl eframe::App for PolygonEditor {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let keys = ctx.input(|input| input.keys_down.clone());
        for key in keys {
            match key {
                egui::Key::Delete => self.run_empty_state(),
                egui::Key::C => self.run_creating_state(),
                _ => break,
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
            AppState::Idle(state) => state.draw(ctx),
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
