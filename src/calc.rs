use egui::{Pos2, Vec2};
use std::f32::consts;

use crate::{
    constants,
    vertex::{Vertex, VertexConstraint},
};

pub fn midpoint(p0: Pos2, p1: Pos2) -> Pos2 {
    Pos2::new((p0.x + p1.x) / 2.0, (p0.y + p1.y) / 2.0)
}

// project p onto the up-diagonal [/] going through s
pub fn project_onto_diagonal_up(s: Pos2, p: Pos2) -> Pos2 {
    let tangent = Vec2::new(consts::SQRT_2 / 2.0, -consts::SQRT_2 / 2.0);
    s + tangent.dot(p - s) * tangent
}

// project p onto the down-diagonal [\] going through s
pub fn project_onto_diagonal_down(s: Pos2, p: Pos2) -> Pos2 {
    let tangent = Vec2::new(consts::SQRT_2 / 2.0, consts::SQRT_2 / 2.0);
    s + tangent.dot(p - s) * tangent
}

// project p onto line ab
pub fn project_onto_line(a: Pos2, b: Pos2, p: Pos2) -> Pos2 {
    let tangent = (b - a).normalized();
    a + tangent.dot(p - a) * tangent
}

pub fn reflection(s: Pos2, p: Pos2) -> Pos2 {
    s - (p - s)
}

// rescale vector sp to length r
pub fn rescale(s: Pos2, p: Pos2, r: f32) -> Pos2 {
    s + (p - s).normalized() * r
}

pub fn are_colinear(a: Pos2, b: Pos2, c: Pos2) -> bool {
    let (v1, v2) = (b - a, c - b);
    (v1.x * v2.y - v2.x * v1.y).abs() < constants::EPS
}

// are p and q symmetric about s
pub fn are_reflections(p: Pos2, q: Pos2, s: Pos2) -> bool {
    let (v1, v2) = (s - p, s - q);
    (v1.x + v2.x).abs() < constants::EPS && (v1.y + v2.y).abs() < constants::EPS
}

// move the free point so that G1 holds
// when goind free -> fixed -> Bézier segment
pub fn keep_g1(fixed: Pos2, free: Pos2, control: Pos2) -> Pos2 {
    let len = (fixed - free).length();
    fixed + len * (fixed - control).normalized()
}

// check if G1 holds when going p0 -> p1 -> Bézier
// (points are colinear and in order)
pub fn check_g1(p0: Pos2, p1: Pos2, control: Pos2) -> bool {
    let (u, v) = ((p0 - p1).normalized(), (control - p1).normalized());
    (u.dot(v) - (-1.0)).abs() < constants::DOT_EPS
}

// move the control point so that G1 holds
// when going p0 -> p1 -> Bézier
pub fn enforce_g1(p0: Pos2, p1: Pos2, control: Pos2) -> Pos2 {
    let tangent = (p1 - p0).normalized();
    p1 + (control - p1).dot(tangent) * tangent
}

// move the free point so that C1 holds
// when going free -> fixed -> Bézier segment
pub fn keep_c1(fixed: Pos2, control: Pos2) -> Pos2 {
    fixed + 3.0 * (fixed - control)
}

// check if C1 holds when going p0 -> p1 -> Bézier
// (points are colinear, in order and distances are in ratio 1:3)
pub fn check_c1(p0: Pos2, p1: Pos2, control: Pos2) -> bool {
    let (u, v) = ((p0 - p1).normalized(), (control - p1).normalized());
    (u.dot(v) - (-1.0)).abs() < constants::EPS
        && (p0.distance(p1) / p1.distance(control) - 3.0).abs() < constants::EPS
}

// move the control point so that C1 holds
// when going p0 -> p1 -> Bézier
pub fn enforce_c1(p0: Pos2, p1: Pos2) -> Pos2 {
    let v = p1 - p0;
    p1 + v / 3.0
}

pub fn bresenham_points(p0: Pos2, p1: Pos2) -> Vec<Pos2> {
    let (mut x0, mut y0, mut x1, mut y1) = (p0.x as i32, p0.y as i32, p1.x as i32, p1.y as i32);
    // if dy > dx, the line goes through octants 2/3/6/7
    // so we switch the axes to make it go through octants 1/4/5/8
    let switch = (y1 - y0).abs() > (x1 - x0).abs();
    if switch {
        (x0, y0) = (y0, x0);
        (x1, y1) = (y1, x1);
    }
    // if x0 > x1, the line goes "backwards", so we switch the endpoints
    if x0 > x1 {
        (x0, x1) = (x1, x0);
        (y0, y1) = (y1, y0);
    }
    // at this point dx >= dy and x1 > x0, so we have a line going through octant 1 or 8

    let (dx, dy) = (x1 - x0, (y1 - y0).abs());
    let mut y = y0;
    let mut d = 2 * dy - dx;
    let incr_e = 2 * dy;
    let incr_ne = 2 * (dy - dx);
    let y_step = if y0 < y1 { 1 } else { -1 };

    let mut points = Vec::new();
    for x in x0..=x1 {
        if switch {
            points.push(Pos2::new(y as f32, x as f32));
        } else {
            points.push(Pos2::new(x as f32, y as f32));
        }

        if d < 0 {
            d += incr_e;
        } else {
            y += y_step;
            d += incr_ne;
        }
    }

    points
}

pub fn cubic_bezier_points(p0: Pos2, p1: Pos2, c0: Pos2, c1: Pos2) -> Vec<Pos2> {
    let (p0, p1, c0, c1) = (p0.to_vec2(), p1.to_vec2(), c0.to_vec2(), c1.to_vec2());
    let a3 = -1.0 * p0 + 3.0 * c0 - 3.0 * c1 + p1;
    let a2 = 3.0 * p0 - 6.0 * c0 + 3.0 * c1;
    let a1 = -3.0 * p0 + 3.0 * c0;
    let a0 = p0;
    let d = constants::BEZIER_DT;

    let delta3 = 6.0 * d * d * d * a3;
    let mut delta2 = 6.0 * d * d * d * a3 + 2.0 * d * d * a2;
    let mut delta = d * d * d * a3 + d * d * a2 + d * a1;
    let mut p = a0;
    let mut points = vec![p.to_pos2()];
    let mut t = 0.0;
    while t <= 1.0 {
        p += delta;
        delta += delta2;
        delta2 += delta3;
        points.push(p.to_pos2());
        t += d;
    }

    points
}

// returns parameters of the arc from edge prev_p-p0 to p1-(somewhere)
// if p0 (the starting vertex) has G1 continuity
pub fn circular_arc_data_with_g1(p0: Pos2, p1: Pos2, prev_p: Pos2) -> (Pos2, f32) {
    let m = midpoint(p0, p1);
    let c = p1 - p0; // vector in the direction of the chord (from p0 to p1)
    let c_n = Vec2::new(-c.y, c.x); // normal to c
    let t = p0 - prev_p; // tangent to the edge prev_p-p0 and to the circle at p0
    let t_n = Vec2::new(-t.y, t.x); // normal to t

    // the center of the circle is the intersection of lines
    // p0 + alpha * t_n and m + beta * c_n (alpha and beta are real parameters)
    // this leads to an equation
    // mat2x2(tx, -cx, ty, -cy) * vec2(alpha, beta) = vec2(mx - p0x, my - p0y)
    // where t_n = [tx, ty], c_n = [cx, cy], m = (mx, my) and p0 = (p0x, p0y)
    // we solve this equation for alpha (and beta)

    let det_den = t_n.x * (-c_n.y) - (-c_n.x) * t_n.y;
    if det_den.abs() < constants::EPS {
        // vectors are parallel, so the intersection point doesn't exist
        return (m, c.length() / 2.0);
    }
    let det_num = (m.x - p0.x) * (-c_n.y) - (-c_n.x) * (m.y - p0.y);
    let alpha = det_num / det_den;

    let s = p0 + alpha * t_n;
    let r = p0.distance(s);

    (s, r)
}

// returns parameters of the arc from edge prev-v0 to v1-next
// the positions of prev and next matter when v0 or v1 have G1 continuity
pub fn circular_arc_data(v0: Vertex, v1: Vertex, prev: Vertex, next: Vertex) -> (Pos2, f32) {
    use VertexConstraint as VC;

    let chord = v1.p - v0.p;
    let m = midpoint(v0.p, v1.p);
    match (v0.vertex_c, v1.vertex_c) {
        (VC::G0, VC::G0) => (m, chord.length() / 2.0),
        (VC::G1, VC::G0) => circular_arc_data_with_g1(v0.p, v1.p, prev.p),
        (VC::G0, VC::G1) => circular_arc_data_with_g1(v1.p, v0.p, next.p),
        // if we somehow manage to have invalid contsraints
        _ => (m, chord.length() / 2.0),
    }
}

// returns points that approximate the arc from p0 to p1 with the center at s and radius r
pub fn arc_points(p0: Pos2, p1: Pos2, s: Pos2, r: f32) -> Vec<Pos2> {
    let mut points = Vec::new();
    let alpha0 = (p0.y - s.y).atan2(p0.x - s.x);
    let alpha1 = (p1.y - s.y).atan2(p1.x - s.x);

    let (mut alpha0, mut alpha1) = (alpha0.min(alpha1), alpha0.max(alpha1));
    if alpha1 - alpha0 > consts::PI {
        let temp = alpha0;
        alpha0 = alpha1;
        alpha1 = temp + 2.0 * consts::PI;
    }
    let mut alpha = alpha0;
    while alpha <= alpha1 {
        points.push(Pos2::new(s.x + r * alpha.cos(), s.y + r * alpha.sin()));
        alpha += constants::ARC_DALPHA;
    }

    points
}
