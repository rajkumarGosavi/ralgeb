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

#[cfg(test)]
mod tests {
  use crate::circle;
  use crate::point;
  fn new_point() -> point::Point {
    point::Point::new(0, 0)
  }
  #[test]
  fn new_circle() {
    let c = circle::Circle::new(1f64, new_point());
    assert_eq!(c.radius, 1f64);
  }
  #[test]
  fn circumference() {
    let c = circle::Circle::new(1 as f64, new_point());
    assert_eq!(
      c.circumference(),
      2 as f64 * c.radius * std::f64::consts::PI
    )
  }
  #[test]
  fn area() {
    let c = circle::Circle::new(1 as f64, new_point());
    assert_eq!(std::f64::consts::PI * c.radius.powi(2), c.area())
  }
}
