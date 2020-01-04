extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;


struct Point(f64, f64);

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point(x, y)
    }

    fn angle_to(&self, p: Point) -> f64 {
        (self.1 - p.1).atan2(self.0 - p.0)
    }

    fn distance_to(&self, p: Point) -> f64 {
        ((p.0 - self.0).powi(2) + (p.1 - self.1).powi(2)).sqrt()
    }

    fn angle(&self) -> f64 {
        self.angle_to(Point::new(0f64, 0f64))
    }

    fn distance(&self) -> f64 {
        self.distance_to(Point::new(0f64, 0f64))
    }
}

#[wasm_bindgen]
pub fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    Point::new(x1, y1).distance_to(Point(x2, y2))
}

#[wasm_bindgen]
pub fn angle(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    Point::new(x1, y1).angle_to(Point::new(x2, y2))
}