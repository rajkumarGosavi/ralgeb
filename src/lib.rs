pub mod line;
pub mod point;

#[cfg(test)]
mod tests {
    use crate::line;
    use crate::point;
    #[test]
    fn point_new() {
        let pt = point::Point::new(10, 20);
        let pt2 = point::Point { x: 10, y: 20 };
        assert_eq!(pt.x, pt2.x);
        assert_eq!(pt.y, pt2.y);
    }
    #[test]
    fn get_origin() {
        let origin = point::Point::get_origin_point();
        let pt2 = point::Point::new(0, 0);
        assert_eq!(origin.x, pt2.x);
        assert_eq!(origin.y, pt2.y);
    }
    #[test]
    fn new_line() {
        let line = line::Line::new(point::Point::new(0, 0), point::Point::new(1, 1));
        print!("{:?}, {:?}", line.point1, line.point2);
    }
    #[test]
    fn line_length() {
        let line = line::Line::new(point::Point::new(0, 0), point::Point::new(1, 1));
        assert_eq!(line.length(), 1.4142135623730951 as f64);
    }
    #[test]
    fn slope() {
        let line = line::Line::new(point::Point::new(0, 0), point::Point::new(1, 1));
        assert_eq!(line.slope(), 1 as f64);
        let line = line::Line::new(point::Point::new(0, 0), point::Point::new(1, 2));
        assert_eq!(line.slope(), 2 as f64);
    }
    #[test]
    fn theta() {
        let line = line::Line::new(point::Point::new(0, 0), point::Point::new(1, 1));
        assert_eq!(line.theta(), 0.7853981633974483_f64);
        let line = line::Line::new(point::Point::new(0, 45), point::Point::new(1, 0));
        assert_eq!(line.theta(), -1.5485777614681775_f64);
    }
}

pub mod utils {
    #[allow(dead_code)]
    /// Returns the signed difference between the x2 and x1 coordinate
    ///
    /// # Arguments
    /// * `x2` - The x coordinate of the final/end point.
    /// * `x1` - The x coordinate of the start/initial point.
    ///
    /// # Examples
    /// ```
    /// use vectorize::point::Point;
    /// use vectorize::utils::delta_x;
    /// let p1 = Point::new(0,10);
    /// let p2 = Point::new(0,9);
    /// delta_x(p2.x, p1.x);
    ///```
    ///
    pub fn delta_x(x2: isize, x1: isize) -> isize {
        x2 - x1
    }
    #[allow(dead_code)]
    /// Returns the signed difference between the y2 and y1 coordinate
    ///
    /// # Arguments
    /// * `y2` - The y coordinate of the final/end point.
    /// * `y1` - The y coordinate of the start/initial point.
    ///
    /// # Examples
    /// ```
    /// use vectorize::point::Point;
    /// use vectorize::utils::delta_y;
    /// let p1 = Point::new(0,10);
    /// let p2 = Point::new(0,9);
    /// delta_y(p2.y, p1.y);
    ///```
    ///
    pub fn delta_y(y2: isize, y1: isize) -> isize {
        y2 - y1
    }
}
