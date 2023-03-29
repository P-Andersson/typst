use super::*;

use crate::eval::Array;
use ecow::{eco_format, EcoString};

use std::cmp::max;
use std::cmp::min;

#[derive(Default, Clone, PartialEq, Hash)]
pub struct Vertex {
    pub x: Scalar,
    pub y: Scalar,
}

impl Vertex {
    pub fn new(x: f64, y: f64) -> Self {
        return Self{x: Scalar::from(x), y: Scalar::from(y)};
    }
}

cast_from_value! {
    Vertex,
    array: Array => {
        let mut iter = array.into_iter();
        match (iter.next(), iter.next(), iter.next()) {
            (Some(a), Some(b), None) => Vertex::new(a.cast()?, b.cast()?) ,
            _ => Err("vertices must contain exactly two entires")?,
        }        
    },
}


/*impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        debug_assert!(!self.x.is_nan(), "float is NaN");
        debug_assert!(!self.y.is_nan(), "float is NaN");
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}*/
/// A polygon of vertices. all points relative and scalable. Non-scalring vertices could
/// be added later
#[derive(Default, Clone, PartialEq, Hash)]
pub struct Polygon {
    pub vertices: Vec<Vertex>,
}

impl Polygon {
    /// Create a new, empty polygon.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Debug for Polygon {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let vertices: Vec<EcoString> = self.vertices.iter().map(|vertex|{
            let x = vertex.x;
            let y = vertex.y;
            return eco_format!("{x:?}, {y:?}");
        }).collect();
        let polygon = vertices.into_iter().reduce(|a, b| a + "; " + b);
        return f.write_str(&polygon.unwrap_or(EcoString::from("()")));
    }
}

cast_from_value! {
    Polygon,
    array: Array => {
        let mut res = Polygon::new();
        for cand in array {
            res.vertices.push(cand.cast()?);
        }
        res
    },
}

cast_to_value! {
    polygon: Polygon => {
        let mut res = Array::new();
        for vertex in &polygon.vertices {
            let mut v = Array::new();
            v.push(Value::Float(vertex.x.0));
            v.push(Value::Float(vertex.y.0));
            res.push(Value::Array(v));
        }
        return Value::Array(res);
    }
}

pub fn polygonal_shape(size: Size, polygon: Polygon, fill: Option<Paint>, stroke: Option<Stroke>) -> Shape {

    let mut minx = Scalar::from(f64::INFINITY);
    let mut miny = Scalar::from(f64::INFINITY);
    let mut maxx = Scalar::from(-f64::INFINITY);
    let mut maxy = Scalar::from(-f64::INFINITY);

    for vertex in &polygon.vertices {
        minx = min(minx, vertex.x);
        miny = min(miny, vertex.y);
        maxx = max(maxx, vertex.x);
        maxy = max(maxy, vertex.y);
    }

    let scalex = size.x.to_raw()/maxx.0 - minx.0;
    let scaley = size.y.to_raw()/maxy.0 - miny.0;
    let offsetx = -minx.0;
    let offsety = -miny.0;

    let point = |x, y| Point::new(Abs::raw((x + offsetx)*scalex), Abs::raw((y + offsety)*scaley));
    let mut path = Path::new();

    let first = polygon.vertices.first();
    if first.is_some() {
        let first_seg = first.unwrap();
        path.move_to(point(first_seg.x.0, first_seg.y.0));
        for vertex in polygon.vertices.iter().skip(1) {
            path.line_to(point(vertex.x.0, vertex.y.0));
        }
        path.close_path();
    }

    Shape { geometry: Geometry::Path(path), stroke, fill }
}
