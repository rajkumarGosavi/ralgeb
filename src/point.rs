use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
///Point represents a unique position in
/// the 2D coordinate system
///
/// # Examples
/// (x: 1,y: 1)
pub struct Point {
  pub x: isize,
  pub y: isize,
}
impl fmt::Display for Point {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}
impl Point {
  #[allow(dead_code)]
  /// Returns a new instance of 2D Point
  ///
  /// # Arguments
  /// * `x` - The x coordinate of the point
  /// * `y` - The y coordinate of the point
  /// ```
  /// use vectorize::point::Point;
  /// let pt = Point::new(1, 1);
  /// ```
  pub fn new(x: isize, y: isize) -> Point {
    Point { x, y }
  }
  #[allow(dead_code)]
  /// Returns a point at the origin
  ///
  /// ```
  /// use vectorize::point::Point;
  /// let pt = Point::get_origin_point();
  /// ```
  pub fn get_origin_point() -> Point {
    Point::new(0, 0)
  }
}
