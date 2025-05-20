/// Checks if all elements in the vector are within the specified bounds.
///
/// # Arguments
///
/// * `values` - A slice of f64 values to check.
/// * `lower_bound` - The lower bound to check against.
/// * `upper_bound` - The upper bound to check against.
///
/// # Returns
///
/// * `true` if all values are within the bounds, `false` otherwise.
pub fn is_vector_within_bounds(values: &[f64], lower_bound: f64, upper_bound: f64) -> bool {
    values
        .iter()
        .all(|&num| num >= lower_bound && num <= upper_bound)
}
