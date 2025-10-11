use crate::polyline::Polyline;

pub struct Polygon {
    pub polyline: Polyline,
}

impl Polygon {
    pub fn new(polyline: Polyline) -> Self {
        Polygon { polyline }
    }
}
