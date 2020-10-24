use std::error::Error;
use std::fmt;

#[derive(Debug)]
/// Represents a rows x cols matrix
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
    ///
    /// # Arguments
    /// `rows` - The number of rows in a matrix.
    /// `cols` - The number of columns in a matrix.
    ///
    /// # Examples
    /// ```
    /// use ralgeb::matrix::Matrix;
    /// let m = Matrix::new(3, 4);
    /// ```
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
    /// Identity matrix are always square matrix
    /// with same number of rows and columns
    ///
    /// # Arguments
    /// `rows` - The number of rows in a matrix.
    /// `cols` - The number of columns in a matrix.
    ///
    /// # Examples
    /// ```
    /// use ralgeb::matrix::Matrix;
    /// let m = match Matrix::identity(3, 3) {
    /// Some(m) => m,
    /// None => Matrix::new(3,3),
    /// };
    /// ```
    ///
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
    ///
    /// # Examples
    /// ```
    /// use ralgeb::matrix::Matrix;
    /// let m = Matrix::new(3,3);
    /// assert_eq!(m.is_square(), true);
    /// ```
    ///
    pub fn is_square(&self) -> bool {
        self.cols == self.rows
    }

    /// Will return a Vector containing all the elements
    /// in the principal diagonal.
    /// Matrix must be a square matrix
    ///
    /// # Examples
    /// ```
    /// use ralgeb::matrix::Matrix;
    /// let m = Matrix::new(3,3);
    /// match m.get_principal() {
    ///  Ok(m) => assert_eq!(m, vec![0.,0.,0.]),
    ///   Err(e) => panic!(e),
    /// }
    ///
    /// ```
    ///
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
    ///
    /// # Arguments
    /// `row_num` - The row number which is to be replaced (starts with 0 index).
    /// `row` - The vector that will replace the row of the matrix.
    ///
    /// # Examples
    /// ```
    /// use ralgeb::matrix::Matrix;
    ///
    /// let m = Matrix::new(3,4);
    /// let m = match m.replace_row(0, vec![1.,2.,3.,4.]) {
    /// Ok(m) => m,
    /// Err(e) => panic!(e),
    /// };
    /// ```
    ///
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
    ///
    /// # Arguments
    /// `row_num` - The row on which the scalar multiplication should occur.
    /// `scalar` - The multiplier. It should be non-zero
    ///
    /// # Examples
    /// ```
    /// use ralgeb::matrix::Matrix;
    ///
    /// let m = Matrix::new(3,4);
    /// let m = match m.scalar_row_mul(2, 7.) {
    /// Ok(m) => m,
    /// Err(e) => panic!(e),
    ///
    /// };
    /// ```
    pub fn scalar_row_mul(mut self, row_num: usize, scalar: f64) -> Result<Matrix, MatrixError> {
        if scalar == 0.0 {
            return Err(MatrixError {
                reason: ErrorCause {
                    cause: format!("The should be non-zero"),
                },
            });
        }
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

    /// Adds the given 2 matrix
    ///
    /// # Arguments
    /// `m1` - 1st matrix
    /// `m2` - 2nd matrix
    ///
    /// # Examples
    /// ```
    /// use ralgeb::matrix::Matrix;
    ///
    /// let m1 = Matrix::new(3,3);
    /// let m2 = Matrix::identity(3,3).unwrap();
    /// let m = match Matrix::add(&m1, &m2) {
    /// Ok(m) => m,
    /// Err(e) => panic!(e),
    /// };
    /// ```
    pub fn add(m1: &Matrix, m2: &Matrix) -> Result<Matrix, MatrixError> {
        if m1.rows == m2.rows && m1.cols == m2.cols {
            let mut res = Matrix::new(m1.rows, m1.cols);
            let mut i = 0;
            while i < m2.rows {
                let mut j = 0;
                while j < m2.cols {
                    res.mat[i][j] = m1.mat[i][j] + m2.mat[i][j];
                    j += 1;
                }
                i += 1;
            }
            Ok(res)
        } else {
            Err(MatrixError {
                reason: ErrorCause {
                    cause: format!(
                        "The dimensions are different. Row Diff: {}, Col Diff: {}",
                        (m1.rows as isize - m2.rows as isize).abs(),
                        (m1.cols as isize - m2.cols as isize).abs()
                    ),
                },
            })
        }
    }
    /// Subtracts the given 2 matrix
    ///
    /// # Arguments
    /// `m1` - 1st matrix
    /// `m2` - 2nd matrix
    ///
    /// # Examples
    /// ```
    /// use ralgeb::matrix::Matrix;
    ///
    /// let m1 = Matrix::new(3,3);
    /// let m2 = Matrix::identity(3,3).unwrap();
    /// let m = match Matrix::subtract(&m1, &m2) {
    /// Ok(m) => m,
    /// Err(e) => panic!(e),
    /// };
    /// ```
    pub fn subtract(m1: &Matrix, m2: &Matrix) -> Result<Matrix, MatrixError> {
        if m1.rows == m2.rows && m1.cols == m2.cols {
            let mut res = Matrix::new(m1.rows, m1.cols);
            let mut i = 0;
            while i < m2.rows {
                let mut j = 0;
                while j < m2.cols {
                    res.mat[i][j] = m1.mat[i][j] - m2.mat[i][j];
                    j += 1;
                }
                i += 1;
            }
            Ok(res)
        } else {
            Err(MatrixError {
                reason: ErrorCause {
                    cause: format!(
                        "The dimensions are different. Row Diff: {}, Col Diff: {}",
                        (m1.rows as isize - m2.rows as isize).abs(),
                        (m1.cols as isize - m2.cols as isize).abs()
                    ),
                },
            })
        }
    }
    /// Transposes a matrix
    ///
    /// # Examples
    /// ```
    /// use ralgeb::matrix::Matrix;
    ///
    /// let mut m = Matrix::identity(3,3).unwrap();
    /// m = Matrix::transpose(m);
    ///
    /// ```
    pub fn transpose(m: Matrix) -> Matrix {
        let mut c = 0;
        let mut mat = Matrix::new(m.cols, m.rows);
        while m.cols > c {
            let mut r = 0;
            let mut v: Vec<f64> = Vec::new();
            while m.rows > r {
                v.push(m.mat[r][c]);
                r += 1;
            }

            mat = mat.replace_row(c, v).unwrap();
            c += 1;
        }
        mat
    }
    /// Multiplies a matrix with a scalar value
    ///
    /// # Arguments
    /// `scalar` - The multiplier. It should be non-zero
    ///
    /// # Examples
    /// ```
    /// use ralgeb::matrix::Matrix;
    ///
    /// let m = Matrix::identity(3,3).unwrap();
    /// let m = match m.scalar_mat_mul(7.) {
    /// Ok(m) => m,
    /// Err(e) => panic!(e),
    /// };
    /// ```
    pub fn scalar_mat_mul(mut self, scalar: f64) -> Result<Matrix, MatrixError> {
        if scalar == 0.0 {
            Err(MatrixError {
                reason: ErrorCause {
                    cause: format!("The should be non-zero"),
                },
            })
        } else {
            let mut r = 0;
            while r < self.rows {
                self = self.scalar_row_mul(r, scalar)?;
                r += 1;
            }
            Ok(Matrix {
                rows: self.rows,
                cols: self.cols,
                mat: (*self.mat).to_vec(),
            })
        }
    }

    /// Will return a row from the matrix
    /// # Arguments
    /// `row_num` - The row number to return. Indexing starts from 0.
    ///
    /// # Examples
    /// ```
    /// use ralgeb::matrix::Matrix;
    /// let m = Matrix::new(3,4);
    /// let row2 = match m.get_row(2) {
    ///   Ok(r) => r,
    ///   Err(e) => panic!(e),
    /// };
    /// ```
    pub fn get_row(&self, row_num: usize) -> Result<Vec<f64>, MatrixError> {
        if row_num >= self.rows {
            Err(MatrixError {
                reason: ErrorCause {
                    cause: format!("The matrix does not contain row: {}", row_num),
                },
            })
        } else {
            Ok(self.mat[row_num].to_owned())
        }
    }

    /// Multiplies two matrices and return the resultant matrix
    /// # Arguments
    /// `m1` - Matrix 1
    /// `m2` - Matrix 2
    ///
    /// # Examples
    /// ```
    /// use ralgeb::matrix::Matrix;
    /// let m1 = Matrix::new(3,4);
    /// let mut m2 = Matrix::new(4,2);
    /// m2 = m2.replace_row(0, vec![1.,2.]).unwrap();
    /// let result = Matrix::multiply(&m1,&m2).unwrap();
    /// assert_eq!(result.rows, 3);
    /// assert_eq!(result.cols, 2);
    /// ```
    ///
    pub fn multiply(m1: &Matrix, m2: &Matrix) -> Result<Matrix, MatrixError> {
        if m1.cols != m2.rows {
            return Err(MatrixError{reason: ErrorCause{cause: format!("The multiplication cannot be performed. The columns of matrix1 {} should be equal to rows of matrix2 {}", m1.cols, m2.rows)}});
        }
        let mut result = Matrix::new(m1.rows, m2.cols);

        for i in 0..m1.rows {
            for j in 0..m2.cols {
                result.mat[i][j] = Matrix::dot_product(&m1.mat[i], &m2.get_col(j));
            }
        }
        Ok(result)
    }
    /// Returns a column from a matrix as a Vector
    ///
    /// # Arguments
    /// `col_num` - The column number to be reeturned (0 indexed)
    ///
    /// # Examples
    ///
    /// ```
    /// use ralgeb::matrix::Matrix;
    ///
    /// let m = Matrix::identity(3, 3).unwrap();
    /// assert_eq!(m.get_col(2), vec![0.,0.,1.]);
    /// ```
    pub fn get_col(&self, col_num: usize) -> Vec<f64> {
        let mut c: Vec<f64> = vec![];
        if col_num >= self.cols {
            vec![0.; self.cols]
        } else {
            for r in self.mat.iter() {
                c.push(r[col_num])
            }
            c
        }
    }
    /// Returns the dot product of 2 vectors.
    /// v1 and v2 should have same length
    /// # Arguments
    /// `v1` - Vector 1 of length n
    /// `v2` - Vector 2 of length n
    ///
    /// # Examples
    ///
    /// ```
    /// use ralgeb::matrix::Matrix;
    /// let v1 = vec![1.,2.,3.];
    /// let v2 = vec![1.,2.,3.];
    /// assert_eq!(Matrix::dot_product(&v1, &v2), 14.);
    /// ```
    ///
    pub fn dot_product(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
        if v1.len() != v2.len() {
            0.
        } else {
            let mut result = 0.;
            for i in 0..v1.len() {
                result += v1[i] * v2[i];
            }
            result
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
    #[test]
    fn add_matrix() {
        let m1 = matrix::Matrix::identity(4, 4).unwrap();
        let m2 = matrix::Matrix::identity(4, 4).unwrap();
        match matrix::Matrix::add(&m1, &m2) {
            Ok(r) => match r.get_principal() {
                Ok(v) => assert_eq!(v, vec![2., 2., 2., 2.]),
                Err(_) => return,
            },
            Err(_) => return,
        }
    }
    #[test]
    fn subtract_matrix() {
        let m1 = matrix::Matrix::identity(4, 4).unwrap();
        let m2 = matrix::Matrix::identity(4, 4).unwrap();
        match matrix::Matrix::subtract(&m1, &m2) {
            Ok(r) => match r.get_principal() {
                Ok(v) => assert_eq!(v, vec![0.; 4]),
                Err(_) => return,
            },
            Err(_) => return,
        }
    }
    #[test]
    fn transpose() {
        let mut m = matrix::Matrix::new(3, 2);
        m = m.replace_row(0, vec![1., 2.]).unwrap();
        m = m.replace_row(1, vec![3., 4.]).unwrap();
        m = m.replace_row(2, vec![5., 6.]).unwrap();
        m = matrix::Matrix::transpose(m);
        let mut m2 = matrix::Matrix::new(2, 3);
        m2 = m2.replace_row(0, vec![1., 3., 5.]).unwrap();
        m2 = m2.replace_row(1, vec![2., 4., 6.]).unwrap();
        assert_eq!(m.cols, m2.cols);
        assert_eq!(m.rows, m2.rows);
        assert_eq!(m.mat, m2.mat);
    }
    #[test]
    fn scalar_mat_mul() {
        let m = matrix::Matrix::identity(3, 3).unwrap();
        assert_eq!(m.scalar_mat_mul(0.).is_err(), true);
        let m = matrix::Matrix::identity(3, 3).unwrap();
        let mut t = matrix::Matrix::new(3, 3);
        t = t.replace_row(0, vec![4., 0., 0.]).unwrap();
        t = t.replace_row(1, vec![0., 4., 0.]).unwrap();
        t = t.replace_row(2, vec![0., 0., 4.]).unwrap();
        match m.scalar_mat_mul(4.) {
            Ok(r) => assert_eq!(r.mat, t.mat),
            Err(_) => return,
        }
    }

    #[test]
    fn get_row() {
        let m = matrix::Matrix::new(3, 4);
        match m.get_row(2) {
            Ok(r) => assert_eq!(r, vec![0.; 4]),
            Err(_) => return,
        }

        assert_eq!(m.get_row(3).is_err(), true);
    }

    #[test]
    fn multiply() {
        let m1 = matrix::Matrix::identity(3, 3).unwrap();
        let mut m2 = matrix::Matrix::new(3, 2);
        m2 = m2.replace_row(0, vec![1., 2.]).unwrap();
        let result = matrix::Matrix::multiply(&m1, &m2).unwrap();
        assert_eq!(result.rows, 3);
        assert_eq!(result.cols, 2);
        assert_eq!(result.get_row(0).unwrap(), vec![1., 2.]);
        assert_eq!(result.get_row(1).unwrap(), vec![0., 0.]);
        assert_eq!(result.get_row(2).unwrap(), vec![0., 0.]);
        println!("Final : {:?}", result.mat);

        let m1 = matrix::Matrix::identity(3, 3).unwrap();
        let m2 = matrix::Matrix::new(2, 2);
        assert_eq!(matrix::Matrix::multiply(&m1, &m2).is_err(), true);
    }
}
