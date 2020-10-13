use crate::point::Point;
use crate::utils::delta_coord;
use std::fmt;
#[derive(Debug)]
// Line is representation of a line in 2D coordinate system with
// each point having x and y coordinates

pub struct Line {
  pub point1: Point,
  pub point2: Point,
}

impl fmt::Display for Line {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[({}) -> ({})]", self.point1, self.point2)
  }
}

impl Line {
  pub fn new(p1: Point, p2: Point) -> Line {
    Line {
      point1: p1,
      point2: p2,
    }
  }
  pub fn length(&self) -> f64 {
    let del_y = self.point2.y - self.point1.y;
    let del_x = self.point2.x - self.point1.x;

    f64::sqrt((del_x.pow(2) + del_y.pow(2)) as f64)
  }
  pub fn slope(&self) -> f64 {
    let del_y = delta_coord(self.point2.y, self.point1.y);
    let del_x = delta_coord(self.point2.x, self.point1.x);
    (del_y / del_x) as f64
  }
  pub fn theta(&self) -> f64 {
    let del_y = delta_coord(self.point2.y, self.point1.y) as f64;
    let del_x = delta_coord(self.point2.x, self.point1.x) as f64;
    del_y.atan2(del_x)
  }
}
