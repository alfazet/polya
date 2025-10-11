use egui::Pos2;

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
