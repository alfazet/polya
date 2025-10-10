use crate::polyline::Polyline;

pub struct Polygon {
    polyline: Polyline,
}

impl Polygon {
    pub fn try_new(polyline: Polyline) -> Option<Self> {
        (polyline.len() >= 3 && polyline.is_closed()).then_some(Polygon { polyline })
    }
}
