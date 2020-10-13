pub mod circle;
pub mod line;
pub mod point;

#[cfg(test)]
mod tests {
    use crate::circle;
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
    #[test]
    fn new_circle() {
        let c = circle::Circle::new(1f64);
        assert_eq!(c.radius, 1f64);
    }
    #[test]
    fn circumference() {
        let c = circle::Circle::new(1 as f64);
        assert_eq!(
            c.circumference(),
            2 as f64 * c.radius * std::f64::consts::PI
        )
    }
    #[test]
    fn area() {
        let c = circle::Circle::new(1 as f64);
        assert_eq!(std::f64::consts::PI * c.radius.powi(2), c.area())
    }
}

pub mod utils {
    /// Returns the signed difference between the x2 and x1 coordinate
    ///
    /// # Arguments
    /// * `x2` - The x coordinate of the final/end point.
    /// * `x1` - The x coordinate of the start/initial point.
    ///
    /// # Examples
    /// ```
    /// use vectorize::utils::delta_coord;
    /// let (x1, x2) = (20, 10);
    /// delta_coord(x2, x1);
    ///```
    ///
    pub fn delta_coord(x2: isize, x1: isize) -> isize {
        x2 - x1
    }
}
