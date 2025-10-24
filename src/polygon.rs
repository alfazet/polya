use egui::{Pos2, Vec2};

use crate::{
    calc, constants,
    vertex::{CircleArc, CubicBezier, EdgeConstraint, Vertex, VertexConstraint},
};

#[derive(Debug)]
pub struct Polygon {
    pub vertices: Vec<Vertex>,
}

impl Polygon {
    pub fn new(vertices: Vec<Vertex>) -> Self {
        Self { vertices }
    }

    fn next_i(&self, i: usize) -> usize {
        (i + 1) % self.vertices.len()
    }

    fn prev_i(&self, i: usize) -> usize {
        (i + self.vertices.len() - 1) % self.vertices.len()
    }

    pub fn edge_len(&self, e_i: usize) -> f32 {
        self.vertices[e_i]
            .p
            .distance(self.vertices[self.next_i(e_i)].p)
    }

    pub fn is_near_edge(&self, v_i: usize, p: Pos2) -> bool {
        let prev_i = self.prev_i(v_i);
        let next_i = self.next_i(v_i);
        let next_next_i = self.next_i(next_i);
        if let Some(bezier) = self.vertices[v_i].bezier {
            calc::cubic_bezier_points(
                self.vertices[v_i].p,
                self.vertices[next_i].p,
                bezier.control[0],
                bezier.control[1],
            )
            .into_iter()
            .any(|b_p| b_p.distance_sq(p) <= constants::SIZE_HITRADIUS * constants::SIZE_HITRADIUS)
        } else if self.vertices[v_i].arc.is_some() {
            let (s, r) = calc::circular_arc_data(
                self.vertices[v_i],
                self.vertices[next_i],
                self.vertices[prev_i],
                self.vertices[next_next_i],
            );
            calc::arc_points(self.vertices[v_i].p, self.vertices[next_i].p, s, r)
                .into_iter()
                .any(|a_p| {
                    a_p.distance_sq(p) <= constants::SIZE_HITRADIUS * constants::SIZE_HITRADIUS
                })
        } else {
            calc::bresenham_points(self.vertices[v_i].p, self.vertices[next_i].p)
                .into_iter()
                .any(|b_p| {
                    b_p.distance_sq(p) <= constants::SIZE_HITRADIUS * constants::SIZE_HITRADIUS
                })
        }
    }

    pub fn has_vertical_neighbor(&self, e_i: usize) -> bool {
        let prev_i = (e_i + self.vertices.len() - 1) % self.vertices.len();
        let next_i = (e_i + 1) % self.vertices.len();

        matches!(self.vertices[prev_i].edge_c, Some(EdgeConstraint::Vertical))
            || matches!(self.vertices[next_i].edge_c, Some(EdgeConstraint::Vertical))
    }

    pub fn is_bezier_start(&self, v_i: usize) -> bool {
        v_i < self.vertices.len() && self.vertices[v_i].bezier.is_some()
    }

    pub fn is_arc_start(&self, v_i: usize) -> bool {
        v_i < self.vertices.len() && self.vertices[v_i].arc.is_some()
    }

    pub fn is_arc_end(&self, v_i: usize) -> bool {
        self.vertices[self.prev_i(v_i)].arc.is_some()
    }

    pub fn apply_constraint(&mut self, v_i: usize) {
        let next_i = self.next_i(v_i);
        let prev_i = self.prev_i(v_i);
        let (p0, p1, p2, p3) = (
            self.vertices[prev_i].p,
            self.vertices[v_i].p,
            self.vertices[next_i].p,
            self.vertices[self.next_i(next_i)].p,
        );
        let p1_c = self.vertices[v_i].vertex_c;
        let p2_c = self.vertices[next_i].vertex_c;
        let prev_bezier = self.vertices[prev_i].bezier;
        let next_bezier = self.vertices[next_i].bezier;
        match &mut self.vertices[v_i].bezier {
            Some(bezier) => {
                if let Some(prev_bezier) = prev_bezier {
                    bezier.control[0] = enforce_joint_continuity(
                        bezier.control[0],
                        prev_bezier.control[1],
                        p1,
                        p1_c,
                    );
                } else {
                    bezier.control[0] =
                        enforce_continuity_constraint(p0, p1, bezier.control[0], p1_c);
                }
                if let Some(next_bezier) = next_bezier {
                    bezier.control[1] = enforce_joint_continuity(
                        bezier.control[1],
                        next_bezier.control[0],
                        p2,
                        p2_c,
                    );
                } else {
                    bezier.control[1] =
                        enforce_continuity_constraint(p3, p2, bezier.control[1], p2_c);
                }
            }
            None => {
                if let Some(c) = self.vertices[v_i].edge_c {
                    let fixed_p = self.vertices[v_i].p;
                    let free_p = self.vertices[next_i].p;
                    match c {
                        EdgeConstraint::Vertical => self.vertices[next_i].p.x = fixed_p.x,
                        EdgeConstraint::DiagonalUp => {
                            self.vertices[next_i].p =
                                calc::project_onto_diagonal_up(fixed_p, free_p);
                        }
                        EdgeConstraint::DiagonalDown => {
                            self.vertices[next_i].p =
                                calc::project_onto_diagonal_down(fixed_p, free_p);
                        }
                        EdgeConstraint::FixedLength(len) => {
                            self.vertices[next_i].p = calc::rescale(fixed_p, free_p, len);
                        }
                    }
                }
            }
        }
    }

    // checks if the constraint is ok for edge v_i-next(v_i)
    pub fn check_constraint(&self, v_i: usize) -> bool {
        let prev_i = self.prev_i(v_i);
        let next_i = self.next_i(v_i);
        match self.vertices[v_i].bezier {
            Some(bezier) => {
                let (p0, p1, p2, p3) = (
                    self.vertices[prev_i].p,
                    self.vertices[v_i].p,
                    self.vertices[next_i].p,
                    self.vertices[self.next_i(next_i)].p,
                );
                // p0 -> p1 -> Bézier -> p2 -> p3
                // so we check if G1/C1 holds
                // when going p0 -> p1 -> Bézier and p3 -> p2 -> Bézier
                let p1_c = self.vertices[v_i].vertex_c;
                let p2_c = self.vertices[next_i].vertex_c;
                let mut res = true;
                if let Some(prev_bezier) = self.vertices[prev_i].bezier {
                    res &=
                        check_joint_continuity(prev_bezier.control[1], bezier.control[0], p1, p1_c);
                } else {
                    res &= check_continuity_constraint(p0, p1, bezier.control[0], p1_c);
                }
                if let Some(next_bezier) = self.vertices[next_i].bezier {
                    res &=
                        check_joint_continuity(bezier.control[1], next_bezier.control[0], p2, p2_c);
                } else {
                    res &= check_continuity_constraint(p3, p2, bezier.control[1], p2_c);
                }

                res
            }
            None => {
                if let Some(c) = self.vertices[v_i].edge_c {
                    let (p0, p1) = (self.vertices[v_i].p, self.vertices[next_i].p);
                    match c {
                        EdgeConstraint::Vertical => (p0.x - p1.x).abs() < constants::EPS,
                        EdgeConstraint::DiagonalUp => {
                            let dy = p0.y - p1.y;
                            let dx = p0.x - p1.x;

                            (dy / dx - (-1.0)).abs() < constants::EPS
                        }
                        EdgeConstraint::DiagonalDown => {
                            let dy = p0.y - p1.y;
                            let dx = p0.x - p1.x;

                            (dy / dx - 1.0).abs() < constants::EPS
                        }
                        EdgeConstraint::FixedLength(len) => {
                            (p0.distance(p1) - len).abs() < constants::DIST_EPS
                        }
                    }
                } else {
                    true
                }
            }
        }
    }

    pub fn resolve_constraints(&mut self, start_i: usize) -> bool {
        for _ in 0..constants::MAX_RESOLVING_ITERS {
            let mut cur_i = start_i;
            loop {
                if !self.check_constraint(cur_i) {
                    self.apply_constraint(cur_i);
                }
                cur_i = self.next_i(cur_i);
                if cur_i == start_i {
                    break;
                }
            }
            cur_i = start_i;
            loop {
                let prev_i = self.prev_i(cur_i);
                if !self.check_constraint(prev_i) {
                    self.apply_constraint(prev_i);
                }
                if prev_i == start_i {
                    break;
                }
                cur_i = prev_i;
            }

            if (0..self.vertices.len()).all(|v_i| self.check_constraint(v_i)) {
                return true;
            }
        }

        false
    }

    // try to move vertex v_i to new_p
    // returns false and rolls back the move
    // if it violated some constraint
    pub fn try_move_vertex(&mut self, v_i: usize, new_p: Pos2) -> bool {
        let backup = self.vertices.clone();
        self.vertices[v_i].p = new_p;

        if self.resolve_constraints(v_i) {
            true
        } else {
            self.vertices = backup;
            false
        }
    }

    // set the given constraint to edge e_i
    // rollback the change if some constraint was violated
    pub fn try_set_edge_constraint(&mut self, e_i: usize, edge_c: EdgeConstraint) {
        let backup = self.vertices.clone();
        self.vertices[e_i].edge_c = Some(edge_c);
        if !self.resolve_constraints(e_i) {
            self.vertices = backup;
        }
    }

    pub fn try_set_vertex_constraint(&mut self, v_i: usize, vertex_c: VertexConstraint) {
        let backup = self.vertices.clone();
        self.vertices[v_i].vertex_c = vertex_c;
        if !self.resolve_constraints(v_i) {
            self.vertices = backup;
        }
    }

    pub fn move_polygon(&mut self, delta: Vec2) {
        for v in self.vertices.iter_mut() {
            v.p += delta;
            if let Some(bezier) = &mut v.bezier {
                bezier.control[0] += delta;
                bezier.control[1] += delta;
            }
        }
    }

    pub fn remove_vertex(&mut self, mut v_i: usize) {
        if self.vertices.len() == 3 {
            return;
        }
        let prev_i = (v_i + self.vertices.len() - 1) % self.vertices.len();
        self.vertices[prev_i].edge_c = None;
        self.vertices[prev_i].bezier = None;
        self.vertices[prev_i].arc = None;
        self.vertices.remove(v_i);
        if v_i == self.vertices.len() {
            v_i = 0;
        }
        // this can't fail since removing a vertex
        // doesn't tighten any constraints
        let _ = self.try_move_vertex(v_i, self.vertices[v_i].p);
    }

    pub fn subdivide_edge(&mut self, e_i: usize) {
        self.vertices[e_i].edge_c = None;
        self.vertices[e_i].bezier = None;
        let next_i = (e_i + 1) % self.vertices.len();
        self.vertices.insert(
            next_i,
            Vertex::new(calc::midpoint(
                self.vertices[e_i].p,
                self.vertices[next_i].p,
            )),
        );
    }

    // initial positions for the control points
    // of a Bézier curve replacing the edge e_i
    fn init_bezier_control_points(&self, e_i: usize) -> [Pos2; 2] {
        let c0 = self.vertices[e_i].p
            + (1.0 / 3.0) * (self.vertices[e_i].p - self.vertices[self.prev_i(e_i)].p);
        let next_i = self.next_i(e_i);
        let c1 = self.vertices[next_i].p
            + (1.0 / 3.0) * (self.vertices[next_i].p - self.vertices[self.next_i(next_i)].p);

        [c0, c1]
    }

    pub fn init_bezier(&mut self, e_i: usize) {
        let control = self.init_bezier_control_points(e_i);
        self.vertices[e_i].bezier = Some(CubicBezier::new(control));
    }

    fn resolve_g1(
        &mut self,
        free_i: usize,
        fixed_i: usize,
        control_p: Pos2,
        edge_c: Option<EdgeConstraint>,
    ) {
        // in general we don't want to move fixed_i, but
        // we need to if the angle is constrained
        match edge_c {
            Some(EdgeConstraint::Vertical) => self.vertices[fixed_i].p.x = control_p.x,
            Some(EdgeConstraint::DiagonalUp) => {
                self.vertices[fixed_i].p =
                    calc::project_onto_diagonal_up(control_p, self.vertices[fixed_i].p)
            }
            Some(EdgeConstraint::DiagonalDown) => {
                self.vertices[fixed_i].p =
                    calc::project_onto_diagonal_down(control_p, self.vertices[fixed_i].p)
            }
            _ => (),
        }
        self.vertices[free_i].p =
            calc::keep_g1(self.vertices[fixed_i].p, self.vertices[free_i].p, control_p);
    }

    // resolves G1 continuity in a joint between Bezier curves
    // (control points need to be colinear with the joint)
    fn resolve_g1_joint(&mut self, v_i: usize, which: usize, joint_i: usize, control_p: Pos2) {
        let joint_p = self.vertices[joint_i].p;
        if let Some(bezier) = &mut self.vertices[v_i].bezier {
            bezier.control[which] =
                calc::project_onto_line(joint_p, control_p, bezier.control[which])
        }
    }

    fn resolve_c1_joint(&mut self, v_i: usize, which: usize, joint_i: usize, control_p: Pos2) {
        let joint_p = self.vertices[joint_i].p;
        if let Some(bezier) = &mut self.vertices[v_i].bezier {
            bezier.control[which] = calc::reflection(joint_p, control_p);
        }
    }

    fn resolve_c1(
        &mut self,
        free_i: usize,
        fixed_i: usize,
        control_p: Pos2,
        edge_c: Option<EdgeConstraint>,
    ) -> Pos2 {
        let mut resolved_p = control_p;
        match edge_c {
            Some(EdgeConstraint::Vertical) => self.vertices[fixed_i].p.x = control_p.x,
            Some(EdgeConstraint::DiagonalUp) => {
                self.vertices[fixed_i].p =
                    calc::project_onto_diagonal_up(control_p, self.vertices[fixed_i].p)
            }
            Some(EdgeConstraint::DiagonalDown) => {
                self.vertices[fixed_i].p =
                    calc::project_onto_diagonal_down(control_p, self.vertices[fixed_i].p)
            }
            // if we have a length constraint on the edge, the control vertex can only be
            // in one place
            Some(EdgeConstraint::FixedLength(len)) => {
                resolved_p = calc::rescale(self.vertices[fixed_i].p, control_p, len / 3.0);
            }
            _ => (),
        }
        self.vertices[free_i].p = calc::keep_c1(self.vertices[fixed_i].p, resolved_p);

        resolved_p
    }

    pub fn resolve_bezier_constraints(&mut self, v_i: usize, which: usize, new_p: Pos2) -> bool {
        let prev_i = self.prev_i(v_i);
        let next_i = self.next_i(v_i);
        let next_next_i = self.next_i(next_i);
        let vertex_c = match which {
            0 => self.vertices[v_i].vertex_c,
            1 => self.vertices[next_i].vertex_c,
            _ => unreachable!(),
        };
        match vertex_c {
            // control point can be wherever
            VertexConstraint::G0 => {
                self.vertices[v_i].move_bezier_control_vertex(which, new_p);
                return true;
            }
            // control point needs to be colinear with the edge that leads into it
            VertexConstraint::G1 => {
                self.vertices[v_i].move_bezier_control_vertex(which, new_p);
                match which {
                    0 => {
                        if self.is_bezier_start(prev_i) {
                            self.resolve_g1_joint(prev_i, 1, v_i, new_p);
                        } else {
                            self.resolve_g1(prev_i, v_i, new_p, self.vertices[prev_i].edge_c);
                        }
                    }
                    1 => {
                        if self.is_bezier_start(next_i) {
                            self.resolve_g1_joint(next_i, 0, next_i, new_p);
                        } else {
                            self.resolve_g1(
                                next_next_i,
                                next_i,
                                new_p,
                                self.vertices[next_i].edge_c,
                            );
                        }
                    }
                    _ => unreachable!(),
                };
            }
            // control point needs to be colinear with the edge that leads into it
            // and dist(p1, control point) = 1/3 * dist(p0, p1)
            VertexConstraint::C1 => match which {
                0 => {
                    if self.is_bezier_start(prev_i) {
                        self.resolve_c1_joint(prev_i, 1, v_i, new_p);
                    } else {
                        let edge_c = self.vertices[prev_i].edge_c;
                        let resolved_p = self.resolve_c1(prev_i, v_i, new_p, edge_c);
                        self.vertices[v_i].move_bezier_control_vertex(0, resolved_p);
                    }
                }
                1 => {
                    if self.is_bezier_start(next_i) {
                        self.resolve_c1_joint(next_i, 0, next_i, new_p);
                    } else {
                        let edge_c = self.vertices[next_i].edge_c;
                        let resolved_p = self.resolve_c1(next_next_i, next_i, new_p, edge_c);
                        self.vertices[v_i].move_bezier_control_vertex(1, resolved_p);
                    }
                }
                _ => unreachable!(),
            },
        }

        self.resolve_constraints(v_i)
    }

    pub fn try_move_control_vertex(&mut self, v_i: usize, which: usize, new_p: Pos2) {
        let backup = self.vertices.clone();
        if !self.resolve_bezier_constraints(v_i, which, new_p) {
            self.vertices = backup;
        }
    }

    pub fn make_arc(&mut self, e_i: usize) {
        use VertexConstraint as VC;

        self.vertices[e_i].arc = Some(CircleArc);
        let next_i = self.next_i(e_i);
        // only G0/G1 is allowed with arcs, and at most one end can be G1
        match (self.vertices[e_i].vertex_c, self.vertices[next_i].vertex_c) {
            (VC::C1, VC::C1) | (VC::C1, VC::G1) | (VC::G1, VC::C1) => {
                self.vertices[e_i].vertex_c = VC::G1;
                self.vertices[next_i].vertex_c = VC::G0;
            }
            (VC::G1, _) => {
                self.vertices[next_i].vertex_c = VC::G0;
            }
            (_, VC::G1) => {
                self.vertices[e_i].vertex_c = VC::G0;
            }
            _ => (),
        }
    }

    pub fn can_be_g1(&self, v_i: usize) -> bool {
        let next_i = self.next_i(v_i);
        let prev_i = self.prev_i(v_i);

        !(self.is_arc_start(v_i) || self.is_arc_end(v_i))
            || (self.is_arc_start(v_i)
                && matches!(self.vertices[next_i].vertex_c, VertexConstraint::G0))
            || (self.is_arc_end(v_i)
                && matches!(self.vertices[prev_i].vertex_c, VertexConstraint::G0))
    }

    pub fn can_be_c1(&self, v_i: usize) -> bool {
        !(self.is_arc_start(v_i) || self.is_arc_end(v_i))
    }
}

fn enforce_continuity_constraint(p0: Pos2, p1: Pos2, control: Pos2, c: VertexConstraint) -> Pos2 {
    match c {
        VertexConstraint::G0 => control,
        VertexConstraint::G1 => calc::enforce_g1(p0, p1, control),
        VertexConstraint::C1 => calc::enforce_c1(p0, p1),
    }
}

fn enforce_joint_continuity(
    control_p: Pos2,
    other_control_p: Pos2,
    v: Pos2,
    c: VertexConstraint,
) -> Pos2 {
    match c {
        VertexConstraint::G0 => control_p,
        VertexConstraint::G1 => calc::project_onto_line(v, other_control_p, control_p),
        VertexConstraint::C1 => calc::reflection(v, other_control_p),
    }
}

fn check_continuity_constraint(p0: Pos2, p1: Pos2, control: Pos2, c: VertexConstraint) -> bool {
    match c {
        VertexConstraint::G0 => true,
        VertexConstraint::G1 => calc::check_g1(p0, p1, control),
        VertexConstraint::C1 => calc::check_c1(p0, p1, control),
    }
}

fn check_joint_continuity(
    control_p1: Pos2,
    control_p2: Pos2,
    v: Pos2,
    c: VertexConstraint,
) -> bool {
    match c {
        VertexConstraint::G0 => true,
        VertexConstraint::G1 => calc::are_colinear(control_p1, control_p2, v),
        VertexConstraint::C1 => calc::are_reflections(control_p1, control_p2, v),
    }
}
