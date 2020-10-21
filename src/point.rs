use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
///Point represents a unique position in
/// the 2D coordinate system
///
/// # Examples
/// (x: 1.,y: 1.)
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl Point {
    /// Returns a new instance of 2D Point
    ///
    /// # Arguments
    /// * `x` - The x coordinate of the point
    /// * `y` - The y coordinate of the point
    /// ```
    /// use vectorize::point::Point;
    /// let pt = Point::new(1., 1.);
    /// ```
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
    /// Returns a point at the origin
    ///
    /// ```
    /// use vectorize::point::Point;
    /// let pt = Point::get_origin_point();
    /// ```
    pub fn get_origin_point() -> Point {
        Point::new(0., 0.)
    }
}

#[cfg(test)]
mod tests {
    use crate::point;
    #[test]
    fn point_new() {
        let pt = point::Point::new(10., 20.);
        let pt2 = point::Point { x: 10., y: 20. };
        assert_eq!(pt.x, pt2.x);
        assert_eq!(pt.y, pt2.y);
    }
    #[test]
    fn get_origin() {
        let origin = point::Point::get_origin_point();
        let pt2 = point::Point::new(0., 0.);
        assert_eq!(origin.x, pt2.x);
        assert_eq!(origin.y, pt2.y);
    }
}
