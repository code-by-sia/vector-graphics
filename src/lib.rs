extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::__rt::std::panic::resume_unwind;

#[wasm_bindgen]
pub struct Point {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
pub struct Line {
    start: Point,
    end: Point,
}

#[wasm_bindgen]
pub struct Ellipse {
    position: Point,
    x_radius: f64,
    y_radius: f64,
}

static ZERO_POINT: Point = Point { x: 0f64, y: 0f64 };

#[wasm_bindgen]
impl Point {
    fn new(x: f64, y: f64) -> Point {
        return Point { x, y };
    }

    pub fn angle_to(&self, p: &Point) -> f64 {
        return (self.y - p.y).atan2(self.x - p.x);
    }

    pub fn distance_to(&self, p: &Point) -> f64 {
        return ((p.x - self.x).powi(2) + (p.y - self.y).powi(2)).sqrt();
    }

    pub fn angle(&self) -> f64 {
        return self.angle_to(&ZERO_POINT);
    }

    pub fn distance(&self) -> f64 {
        return self.distance_to(&ZERO_POINT);
    }

    pub fn rotate(&self, r: f64) -> Point {
        return self.rotate_on(r, &ZERO_POINT);
    }

    pub fn translate(&self, point: &Point) -> Point {
        return Point::new(self.x + point.x, self.y + point.y);
    }

    pub fn scale(&self, factor: f64) -> Point {
        return Point::new(self.x * factor, self.y * factor);
    }

    pub fn rotate_on(&self, r: f64, origin: &Point) -> Point {
        let distance = origin.distance();
        let angle = origin.angle() + r;
        return create_point(angle.cos() * distance, angle.sin() * distance);
    }

    pub fn clone(&self) -> Point {
        return Point::new(self.x, self.y);
    }
}

#[wasm_bindgen]
impl Line {
    fn new(start: Point, end: Point) -> Line {
        return Line { start, end };
    }

    pub fn length(&self) -> f64 {
        return self.start.distance_to(&self.end);
    }

    pub fn angle(&self) -> f64 {
        return self.start.angle_to(&self.end);
    }

    pub fn is_hit(&self, p: &Point) -> bool {
        let start_to_point = &self.start.distance_to(p);
        let end_to_point = &self.start.distance_to(p);
        return start_to_point + end_to_point == self.length();
    }

    pub fn intersect(&self, line: &Line) -> Option<Point> {
        let x1 = &self.start.x;
        let y1 = &self.start.y;
        let x2 = &self.end.x;
        let y2 = &self.end.y;

        let x3 = &line.start.x;
        let y3 = &line.start.y;
        let x4 = &line.end.x;
        let y4 = &line.end.y;

        let denim: f64 = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
        if denim == 0f64 {
            return None;
        }

        let _ua = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / &denim;
        let _ub = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3)) / &denim;

        return Some(Point {
            x: x1 + _ua * (x2 - x1),
            y: y1 + _ua * (y2 - y1),
        });
    }

    pub fn angle_to(&self, line: &Line) -> f64 {
        return self.angle() - line.angle();
    }

    pub fn middle(&self) -> Point {
        return Point::new((self.start.x + self.end.x) / 2f64, (self.start.y + self.end.y) / 2f64);
    }

    pub fn rotate(&self, r: f64) -> Line {
        let middle = self.middle();
        return self.rotate_on(r, &middle);
    }

    pub fn rotate_on(&self, r: f64, origin: &Point) -> Line {
        let start = self.start.rotate_on(r, origin);
        let end = self.end.rotate_on(r, origin);
        return Line::new(start, end);
    }

    pub fn translate(&self, diff: &Point) -> Line {
        return Line::new(self.start.translate(diff), self.end.translate(diff));
    }

    pub fn scale(&self, factor: f64) -> Line {
        return Line::new(self.start.clone(), self.end.scale(factor));
    }

    pub fn clone(&self) -> Line {
        let start = self.start.clone();
        let end = self.end.clone();
        return Line::new(start, end);
    }
}

#[wasm_bindgen]
impl Ellipse {
    fn new(position: Point, x_radius: f64, y_radius: f64) -> Ellipse {
        return Ellipse {
            position,
            x_radius,
            y_radius,
        };
    }

    fn translate(&self, diff: &Point) -> Ellipse {
        return Ellipse::new(self.position.translate(diff), self.x_radius, self.y_radius);
    }

    fn scale(&self, factor: f64) -> Ellipse {
        return Ellipse::new(self.position.clone(), self.x_radius * factor, self.y_radius * factor);
    }

    fn is_hit(&self, point: &Point) -> bool {
        let base = point.translate(&self.position.scale(-1f64));
        let angle = base.angle();
        return (angle.cos() * self.x_radius == point.x) &&
            (angle.sin() * self.y_radius == point.y);
    }
}

#[wasm_bindgen]
pub fn create_point(x: f64, y: f64) -> Point {
    return Point { x, y };
}

#[wasm_bindgen]
pub fn create_line(x1: f64, y1: f64, x2: f64, y2: f64) -> Line {
    let start = create_point(x1, y1);
    let end = create_point(x2, y2);
    return Line::new(start, end);
}

#[wasm_bindgen]
pub fn create_ellipse(position: Point, x_radius: f64, y_radius: f64) -> Ellipse {
    return Ellipse::new(position, x_radius, y_radius);
}