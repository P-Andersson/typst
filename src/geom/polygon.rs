use super::*;

use std::cmp::max;
use std::cmp::min;

pub fn polygon(size: Size, segments: Vec<Point>, fill: Option<Paint>, stroke: Option<Stroke>) -> Shape {

    let mut minx = Abs::inf();
    let mut miny = Abs::inf();
    let mut maxx = -Abs::inf();
    let mut maxy = -Abs::inf();

    for segment in &segments {
        minx = min(minx, segment.x);
        miny = min(miny, segment.y);
        maxx = max(maxx, segment.x);
        maxy = max(maxy, segment.y);
    }

    let scalex = size.x.to_raw()/maxx.to_raw() - minx.to_raw();
    let scaley = size.y.to_raw()/maxy.to_raw() - miny.to_raw();
    let offsetx = -minx;
    let offsety = -miny;

    let point = |x, y| Point::new((x + offsetx)*scalex, (y + offsety)*scaley);
    let mut path = Path::new();

    let first = segments.first();
    if first.is_some() {
        let first_seg = first.unwrap();
        path.move_to(point(first_seg.x, first_seg.y));
        for segment in segments.iter().skip(1) {
            path.line_to(point(segment.x, segment.y));
        }
        path.close_path();
    }


    /*let z = Abs::zero();
    let rx = size.x / 2.0;
    let ry = size.y / 2.0;
    let m = 0.551784;
    let mx = m * rx;
    let my = m * ry;
    let point = |x, y| Point::new(x + rx, y + ry);

    let mut path = Path::new();
    path.move_to(point(-rx, z));
    path.line_to(point(-rx, -my));
    path.line_to(point(mx, -ry));
    path.line_to(point(rx, my));
    path.line_to(point(-mx, ry));*/

    Shape { geometry: Geometry::Path(path), stroke, fill }
}
