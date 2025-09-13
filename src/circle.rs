// Use crate:: to refer to modules declared in main.rs
use crate::shapes::shape::Shape;

pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
    
    fn perimeter(&self) -> f64 { // Must match the trait method name exactly
        2.0 * std::f64::consts::PI * self.radius
    } 
}