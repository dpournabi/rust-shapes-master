// Use crate:: to refer to modules declared in main.rs
use crate::shapes::shape::Shape;

pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn perimeter(&self) -> f64 {
        (2.0 * self.width) + (2.0 * self.height)
    } 
}