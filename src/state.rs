use egui::{Button, Color32, Context, Modifiers, PointerButton, Pos2, Rect, Vec2};

use crate::{
    calc, constants,
    dialog::FixedLengthDialog,
    polygon::Polygon,
    vertex::{CubicBezier, EdgeConstraint, Vertex, VertexConstraint},
};

#[derive(Debug, Default)]
pub struct CreatingState {
    pub vertices: Vec<Vertex>,
}

#[derive(Debug)]
pub struct EditingState {
    pub polygon: Polygon,
    pub dragged_vertex_i: Option<usize>,
    // edge index, which control vertex of this edge (0/1)
    pub dragged_control_vertex_i: Option<(usize, usize)>,
    pub drag_anchor_i: Option<usize>,
    pub selected_vertex_i: Option<usize>,
    pub selected_edge_i: Option<usize>,
    pub fixed_length_dialog: FixedLengthDialog,
}

#[derive(Clone, Copy, Debug)]
pub enum StateTransition {
    ToCreating,
    ToEditing,
}

impl CreatingState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_add_point(
        &mut self,
        ctx: &Context,
        canvas_rect: Rect,
    ) -> Option<StateTransition> {
        if let Some(mouse_pos) = ctx.pointer_interact_pos()
            && canvas_rect.contains(mouse_pos)
            && ctx.input(|i| i.pointer.button_released(PointerButton::Primary))
        {
            if self.vertices.len() >= 3 && self.vertices[0].is_near(mouse_pos) {
                return Some(StateTransition::ToEditing);
            }
            self.vertices.push(Vertex::new(mouse_pos));
        }

        None
    }
}

impl EditingState {
    pub fn new(vertices: Vec<Vertex>) -> Self {
        Self {
            polygon: Polygon::new(vertices),
            dragged_vertex_i: None,
            dragged_control_vertex_i: None,
            drag_anchor_i: None,
            selected_vertex_i: None,
            selected_edge_i: None,
            fixed_length_dialog: FixedLengthDialog::default(),
        }
    }

    pub fn new_predefined() -> Self {
        let mut vertices = vec![
            Vertex::from((300.0, 200.0)),
            Vertex::from((500.0, 200.0)),
            Vertex::from((400.0, 300.0)),
            Vertex::from((400.0, 400.0)),
        ];
        let mut state = Self::new(vertices);
        state.polygon.init_bezier(0);
        state.polygon.vertices[2].edge_c = Some(EdgeConstraint::Vertical);

        state
    }

    pub fn handle_drag_vertex(&mut self, ctx: &Context) {
        if let Some(mouse_pos) = ctx.pointer_interact_pos()
            && ctx.input(|i| {
                i.pointer.button_down(PointerButton::Primary)
                    && i.modifiers.matches_exact(Modifiers::NONE)
            })
        {
            if let Some(v_i) = self.dragged_vertex_i {
                if !self.polygon.try_move_vertex(v_i, mouse_pos) {
                    self.polygon
                        .move_polygon(mouse_pos - self.polygon.vertices[v_i].p);
                }
            } else if let Some((v_i, which)) = self.dragged_control_vertex_i {
                if self.polygon.vertices[v_i].bezier.is_some() {
                    self.polygon.try_move_control_vertex(v_i, which, mouse_pos);
                }
            } else {
                // start dragging
                for (i, v) in self.polygon.vertices.iter().enumerate() {
                    if v.is_near(mouse_pos) {
                        self.dragged_vertex_i = Some(i);
                        break;
                    }
                    if let Some(bezier) = v.bezier
                        && let Some(which) = bezier.nearby_control_vertex(mouse_pos)
                    {
                        self.dragged_control_vertex_i = Some((i, which));
                        break;
                    }
                }
            }
        } else {
            self.dragged_vertex_i = None;
            self.dragged_control_vertex_i = None;
        }
    }

    pub fn handle_drag_polygon(&mut self, ctx: &Context) {
        if let Some(mouse_pos) = ctx.pointer_interact_pos()
            && ctx.input(|i| {
                i.pointer.button_down(PointerButton::Primary)
                    && i.modifiers.matches_exact(Modifiers::SHIFT)
            })
        {
            if let Some(v_i) = self.drag_anchor_i {
                self.polygon
                    .move_polygon(mouse_pos - self.polygon.vertices[v_i].p);
            } else if let Some(v_i) = self
                .polygon
                .vertices
                .iter()
                .position(|v| v.is_near(mouse_pos))
            {
                self.drag_anchor_i = Some(v_i);
            }
        } else {
            self.drag_anchor_i = None;
        }
    }

    pub fn handle_select(&mut self, ctx: &Context) {
        if let Some(mouse_pos) = ctx.pointer_interact_pos()
            && ctx.input(|i| i.pointer.button_down(PointerButton::Secondary))
        {
            self.selected_vertex_i = self
                .polygon
                .vertices
                .iter()
                .position(|v| v.is_near(mouse_pos));
            if self.selected_vertex_i.is_some() {
                self.selected_edge_i = None;
                return;
            }
            self.selected_edge_i = self
                .polygon
                .vertices
                .iter()
                .enumerate()
                .position(|(i, _)| self.polygon.is_near_edge(i, mouse_pos));
            if self.selected_edge_i.is_some() {
                self.selected_vertex_i = None;
            }
        }
    }

    pub fn handle_vertex_context_menu(&mut self, ctx: &Context) {
        let Some(v_i) = self.selected_vertex_i else {
            return;
        };
        let menu_pos =
            self.polygon.vertices[v_i].p + Vec2::splat(constants::SIZE_CONTEXT_MENU_OFFSET);
        // let has_continuity_options = self.polygon.is_bezier_start(v_i)
        //     || self.polygon.is_bezier_end(v_i)
        //     || self.polygon.is_arc_start(v_i)
        //     || self.polygon.is_arc_end(v_i);
        egui::containers::Area::new(constants::ID_VERTEX_CONTEXT_MENU.into())
            .fixed_pos(menu_pos)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style())
                    .outer_margin(0.0)
                    .inner_margin(0.0)
                    .fill(Color32::TRANSPARENT)
                    .show(ui, |ui| {
                        ui.set_min_width(constants::SIZE_CONTEXT_MENU);
                        ui.spacing_mut().item_spacing = Vec2::ZERO;
                        ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                            if ui.add(Button::new("Remove")).clicked() {
                                self.polygon.remove_vertex(v_i);
                                self.selected_vertex_i = None;
                            }
                            if ui.add(Button::new("Set G0")).clicked() {
                                self.polygon
                                    .try_set_vertex_constraint(v_i, VertexConstraint::G0);
                                self.selected_vertex_i = None;
                            }
                            if ui
                                .add_enabled(self.polygon.can_be_g1(v_i), Button::new("Set G1"))
                                .clicked()
                            {
                                self.polygon
                                    .try_set_vertex_constraint(v_i, VertexConstraint::G1);
                                self.polygon
                                    .try_move_vertex(v_i, self.polygon.vertices[v_i].p);
                                self.selected_vertex_i = None;
                            }
                            if ui
                                .add_enabled(self.polygon.can_be_c1(v_i), Button::new("Set C1"))
                                .clicked()
                            {
                                self.polygon
                                    .try_set_vertex_constraint(v_i, VertexConstraint::C1);
                                self.polygon
                                    .try_move_vertex(v_i, self.polygon.vertices[v_i].p);
                                self.selected_vertex_i = None;
                            }
                        });
                    });
            });
    }

    pub fn handle_edge_context_menu(&mut self, ctx: &Context) {
        let Some(e_i) = self.selected_edge_i else {
            return;
        };
        const CONSTRAINED: u8 = 1;
        const BEZIER: u8 = 2;
        const ARC: u8 = 4;
        let mut mask = 0;
        if self.polygon.vertices[e_i].edge_c.is_some() {
            mask |= CONSTRAINED;
        }
        if self.polygon.vertices[e_i].bezier.is_some() {
            mask |= BEZIER;
        }
        if self.polygon.vertices[e_i].arc.is_some() {
            mask |= ARC;
        }

        let next_i = (e_i + 1) % self.polygon.vertices.len();
        let menu_pos = calc::midpoint(
            self.polygon.vertices[e_i].p,
            self.polygon.vertices[next_i].p,
        ) + Vec2::splat(constants::SIZE_CONTEXT_MENU_OFFSET);
        egui::containers::Area::new(constants::ID_EDGE_CONTEXT_MENU.into())
            .fixed_pos(menu_pos)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style())
                    .outer_margin(0.0)
                    .inner_margin(0.0)
                    .fill(Color32::TRANSPARENT)
                    .show(ui, |ui| {
                        ui.set_min_width(constants::SIZE_CONTEXT_MENU);
                        ui.spacing_mut().item_spacing = Vec2::ZERO;
                        ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                            if mask == 0 {
                                if ui
                                    .add_enabled(
                                        !self.polygon.has_vertical_neighbor(e_i),
                                        Button::new("Make vertical"),
                                    )
                                    .clicked()
                                {
                                    self.polygon
                                        .try_set_edge_constraint(e_i, EdgeConstraint::Vertical);
                                    self.selected_edge_i = None;
                                }
                                if ui.add(Button::new("Make diagonal up [/]")).clicked() {
                                    self.polygon
                                        .try_set_edge_constraint(e_i, EdgeConstraint::DiagonalUp);
                                    self.selected_edge_i = None;
                                }
                                if ui.add(Button::new("Make diagonal down [\\]")).clicked() {
                                    self.polygon
                                        .try_set_edge_constraint(e_i, EdgeConstraint::DiagonalDown);
                                    self.selected_edge_i = None;
                                }
                                if ui.add(Button::new("To Bézier segment")).clicked() {
                                    self.polygon.init_bezier(e_i);
                                    self.polygon.vertices[e_i].edge_c = None;
                                    self.polygon
                                        .try_move_vertex(e_i, self.polygon.vertices[e_i].p);
                                    self.selected_edge_i = None;
                                }
                                if ui.add(Button::new("To circular arc")).clicked() {
                                    self.polygon.make_arc(e_i);
                                    self.polygon.vertices[e_i].edge_c = None;
                                    self.polygon
                                        .try_move_vertex(e_i, self.polygon.vertices[e_i].p);
                                    self.selected_edge_i = None;
                                }
                                let fix_length_btn = ui.add(Button::new("Fix length"));
                                self.fixed_length_dialog.render(ui, &fix_length_btn);
                                if fix_length_btn.clicked() {
                                    self.fixed_length_dialog
                                        .open(ui, self.polygon.edge_len(e_i));
                                }
                                if self.fixed_length_dialog.applied {
                                    let len = self.fixed_length_dialog.value;
                                    self.polygon.try_set_edge_constraint(
                                        e_i,
                                        EdgeConstraint::FixedLength(len),
                                    );
                                    self.selected_edge_i = None;
                                    self.fixed_length_dialog.applied = false;
                                }
                            }
                            if ((mask & BEZIER) | (mask & ARC)) == 0 {
                                if ui.add(Button::new("Subdivide")).clicked() {
                                    self.polygon.subdivide_edge(e_i);
                                    self.selected_edge_i = None;
                                }
                            }
                            if (mask & CONSTRAINED) > 0 {
                                if ui.add(Button::new("Remove constraint")).clicked() {
                                    self.polygon.vertices[e_i].edge_c = None;
                                    self.selected_edge_i = None;
                                }
                            }
                            if (mask & BEZIER) > 0 {
                                if ui.add(Button::new("Remove Bézier segment")).clicked() {
                                    self.polygon.vertices[e_i].bezier = None;
                                    self.selected_edge_i = None;
                                }
                            }
                            if (mask & ARC) > 0 {
                                if ui.add(Button::new("Remove arc")).clicked() {
                                    self.polygon.vertices[e_i].arc = None;
                                    self.selected_edge_i = None;
                                }
                            }
                        });
                    });
            });
    }
}
