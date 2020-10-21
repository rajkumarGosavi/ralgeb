/// Returns the signed difference between the x2 and x1 coordinate
///
/// # Arguments
/// * `x2` - The x coordinate of the final/end point.
/// * `x1` - The x coordinate of the start/initial point.
///
/// # Examples
/// ```
/// use vectorize::utils::delta_coord;
/// let (x1, x2) = (20., 10.);
/// delta_coord(x2, x1);
///```
///
pub fn delta_coord(x2: f64, x1: f64) -> f64 {
    x2 - x1
}
