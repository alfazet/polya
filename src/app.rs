use eframe::egui;

use crate::{
    constants,
    render::{self, LineAlgorithm},
    state::{CreatingState, EditingState, StateTransition},
};

#[derive(Debug)]
enum AppState {
    Creating(CreatingState),
    Editing(EditingState),
}

pub struct App {
    state: AppState,
    line_algo: LineAlgorithm,
}

impl Default for App {
    fn default() -> Self {
        let e_state = EditingState::new_predefined();

        Self {
            state: AppState::Editing(e_state),
            line_algo: LineAlgorithm::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        ctx.set_zoom_factor(1.5);
        egui::SidePanel::left(constants::ID_SIDEBAR_LEFT)
            .resizable(false)
            .frame(
                egui::Frame::new()
                    .fill(constants::COLOR_BKG)
                    .inner_margin(egui::Margin::same(constants::SIZE_MARGIN)),
            )
            .show(ctx, |ui| {
                ui.label("Line rendering");
                ui.radio_value(&mut self.line_algo, LineAlgorithm::Default, "Default");
                ui.radio_value(&mut self.line_algo, LineAlgorithm::Bresenham, "Bresenham");
                ui.separator();
                ui.vertical_centered(|ui| {
                    if ui.button("Reset").clicked() {
                        *self = Self::reset(self.line_algo);
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().interaction.selectable_labels = false;
            match self.state {
                AppState::Creating(_) => {
                    ui.vertical_centered(|ui| {
                        ui.heading("Create a polygon to start editing it");
                        ui.weak("Add vertices with LMB");
                    });
                }
                AppState::Editing(_) => {
                    ui.vertical_centered(|ui| {
                        ui.heading("Edit the polygon and move it around");
                        ui.weak("Move vertices and Bézier control points by dragging");
                        ui.weak("Move the entire polygon instead by holding [Shift]");
                        ui.weak("Toggle constraints with RMB on vertex/edge");
                    });
                }
            }
            let painter = ui.painter();
            match &mut self.state {
                AppState::Creating(c_state) => {
                    render::render_polyline_edges(painter, &c_state.vertices, self.line_algo);
                    render::render_vertices(painter, &c_state.vertices, Some(0), false);

                    if let Some(trans) = c_state.handle_add_point(ctx, ui.min_rect())
                        && let StateTransition::ToEditing = trans
                    {
                        self.state = AppState::Editing(EditingState::new(std::mem::take(
                            &mut c_state.vertices,
                        )));
                    }
                }
                AppState::Editing(e_state) => {
                    render::render_polygon_edges(
                        painter,
                        &e_state.polygon.vertices,
                        e_state.selected_edge_i,
                        self.line_algo,
                    );
                    render::render_vertices(
                        painter,
                        &e_state.polygon.vertices,
                        e_state.selected_vertex_i,
                        true,
                    );

                    e_state.handle_drag_vertex(ctx);
                    e_state.handle_drag_polygon(ctx);
                    e_state.handle_select(ctx);
                    // handle_select before doing actions that depend on the current selection
                    e_state.handle_vertex_context_menu(ctx);
                    e_state.handle_edge_context_menu(ctx);
                }
            }
        });
    }
}

impl App {
    fn reset(line_algo: LineAlgorithm) -> Self {
        let c_state = CreatingState::new();

        Self {
            state: AppState::Creating(c_state),
            line_algo,
        }
    }
}
