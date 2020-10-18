use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Matrix {
  pub rows: usize,
  pub cols: usize,
  mat: Vec<Vec<f64>>,
}

#[derive(Debug)]
pub struct MatrixError {
  reason: ErrorCause,
}

impl fmt::Display for MatrixError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Matrix Error: {}", self.reason)
  }
}

impl Error for MatrixError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    Some(&self.reason)
  }
}

#[derive(Debug)]
pub struct ErrorCause {
  cause: String,
}

impl fmt::Display for ErrorCause {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Cause: ")
  }
}

impl Error for ErrorCause {}

impl Matrix {
  /// Returns a matrix with all 0 values
  pub fn new(rows: usize, cols: usize) -> Matrix {
    let mut r = 0;
    let mut outer_vec: Vec<Vec<f64>> = Vec::new();
    loop {
      let inner_vec: Vec<f64> = vec![0.0; cols];
      if r >= rows {
        break Matrix {
          rows: outer_vec.len(),
          cols: inner_vec.len(),
          mat: outer_vec,
        };
      }
      outer_vec.push(inner_vec);
      r += 1;
    }
  }
  /// Returns an Identity matrix
  pub fn identity(rows: usize, cols: usize) -> Option<Matrix> {
    if rows == cols {
      let mut m = Matrix::new(rows, cols);
      let mut r = 0;
      loop {
        if r >= rows {
          break;
        }
        m.mat[r][r] = 1.0;
        r += 1;
      }
      Some(m)
    } else {
      None
    }
  }
  /// is_square takes in a Matrix and returns whether
  /// the matrix is a square matrix i.e nxn
  /// or not
  pub fn is_square(&self) -> bool {
    self.cols == self.rows
  }

  /// Will return a Vector containing all the elements
  /// in the principal diagonal
  pub fn get_principal(&self) -> Result<Vec<f64>, MatrixError> {
    let mut principal: Vec<f64> = Vec::new();
    if self.is_square() {
      let mut r = 0;
      loop {
        if r >= self.rows {
          break;
        } else {
          principal.push(self.mat[r][r]);
          r += 1;
        }
      }
      Ok(principal)
    } else {
      Err(MatrixError {
        reason: ErrorCause {
          cause: format!("The matrix is not a square matrix"),
        },
      })
    }
  }

  /// Replaces a row with a new row.
  pub fn replace_row(mut self, row_num: usize, row: Vec<f64>) -> Result<Matrix, MatrixError> {
    if self.cols != row.len() {
      Err(MatrixError {
        reason: ErrorCause {
          cause: format!(
            "The number of columns differ by {}",
            ((self.cols as f64 - row.len() as f64) as f64).abs()
          ),
        },
      })
    } else {
      self.mat[row_num] = row;
      Ok(Matrix {
        rows: self.rows,
        cols: self.cols,
        mat: self.mat,
      })
    }
  }
  /// Multiplies a row of matrix with a scalar value
  pub fn scalar_row_mul(mut self, row_num: usize, scalar: f64) -> Result<Matrix, MatrixError> {
    if row_num <= self.rows {
      self.mat[row_num] = self.mat[row_num].iter().map(|x| x * scalar).collect();
      Ok(Matrix {
        rows: self.rows,
        cols: self.cols,
        mat: self.mat,
      })
    } else {
      Err(MatrixError {
        reason: ErrorCause {
          cause: format!("The row {} does not exists", row_num),
        },
      })
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::matrix;
  #[test]
  fn new_zero_matrix() {
    let m = matrix::Matrix::new(2, 3);
    let v: Vec<f64> = vec![0.0; 3];
    assert_eq!(m.mat[0], v);
    assert_eq!(m.rows, 2);
    assert_eq!(m.cols, 3);
  }

  #[test]
  fn identity() {
    // Identity matrix is always square matrix

    // Provide rectangular row and col
    let m = matrix::Matrix::identity(4, 3);
    assert_eq!(true, m.is_none());

    // Provide square row and col
    let m = matrix::Matrix::identity(3, 3);
    assert_eq!(m.is_some(), true);
    match m {
      Option::Some(m) => assert_eq!(m.mat[0], vec![1., 0., 0.]),
      _ => assert_eq!(m.is_none(), false),
    };
  }
  #[test]
  fn replace_row() {
    // Should return matrix with replaced row
    let m = matrix::Matrix::new(1, 3);
    let v: Vec<f64> = vec![1.0, 2.0, 3.0];
    let t = m.replace_row(0, v);
    match t {
      Ok(m) => assert_eq!(m.mat[0], vec![1., 2., 3.]),
      _ => assert_eq!(t.is_err(), false),
    }

    // When error occurs
    let m = matrix::Matrix::new(1, 1);
    let v: Vec<f64> = vec![1.0, 2.0, 3.0];
    assert_eq!(m.replace_row(0, v).is_err(), true);
  }

  #[test]
  fn scalar_row_mul() {
    let m = matrix::Matrix::identity(3, 3);
    match m {
      Some(m) => match m.scalar_row_mul(1, 3.0) {
        Ok(r) => {
          let v: Vec<f64> = vec![0.0, 3.0, 0.0];
          assert_eq!(r.mat[1], v);
        }
        Err(e) => println!("{:?}", e),
      },
      None => println!("Nothing found"),
    }

    let m = matrix::Matrix::identity(3, 3);
    match m {
      Some(m) => {
        assert_eq!(m.scalar_row_mul(4, -0.3).is_err(), true);
      }
      None => println!("Nothing found"),
    }
  }
  #[test]
  fn get_principal() {
    match matrix::Matrix::identity(4, 4) {
      None => return,
      Some(m) => match m.get_principal() {
        Err(_) => return,
        Ok(m) => {
          assert_eq!(m, vec![1., 1., 1., 1.]);
        }
      },
    }
    let m = matrix::Matrix::new(4, 3);
    assert_eq!(m.get_principal().is_err(), true);
  }
}
