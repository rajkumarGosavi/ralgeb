use crate::point::Point;
pub struct Circle {
  pub radius: f64,
  pub centre: Point,
}

impl Circle {
  pub fn new(r: f64, centre: Point) -> Circle {
    Circle { radius: r, centre }
  }
  pub fn circumference(&self) -> f64 {
    2 as f64 * self.radius * std::f64::consts::PI
  }
  pub fn area(&self) -> f64 {
    std::f64::consts::PI * self.radius.powi(2)
  }
}
