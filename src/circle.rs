use crate::point::Point;

/// Represents a circle
pub struct Circle {
    pub radius: f64,
    pub centre: Point,
}

impl Circle {
    /// Creates a new instance of circle
    /// with given radius and centre point
    /// # Arguments
    /// `r` - radius of the circle
    /// `centre` - The point specifying the centre of circle in 2D coordinate
    ///
    /// # Examples
    /// ```
    /// use vectorize::point::Point;
    /// use vectorize::circle::Circle;
    /// let c = Circle::new(2., Point{x: 3.,y: -4.});
    /// ```
    pub fn new(r: f64, centre: Point) -> Circle {
        Circle { radius: r, centre }
    }

    /// Returns the circumference of the circle
    ///
    /// # Examples
    /// ```
    /// use vectorize::point::Point;
    /// use vectorize::circle::Circle;
    /// let c = Circle::new(2., Point{x: 3.,y: -4.});
    /// assert_eq!(c.circumference(), 2. * 2. * std::f64::consts::PI);
    /// ```
    pub fn circumference(&self) -> f64 {
        2 as f64 * self.radius * std::f64::consts::PI
    }
    /// Returns the area of the circle
    ///
    /// # Examples
    /// ```
    /// use vectorize::point::Point;
    /// use vectorize::circle::Circle;
    /// let c = Circle::new(2., Point{x: 3.,y: -4.});
    /// assert_eq!(c.area(), std::f64::consts::PI * (2 as f64).powi(2));
    /// ```
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

#[cfg(test)]
mod tests {
    use crate::circle;
    use crate::point;
    fn new_point() -> point::Point {
        point::Point::new(0., 0.)
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
