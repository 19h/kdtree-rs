//! Defines different distance metrics, in simplest case it defines the
//! euclidean distance which is no more than the square root of the sum of the
//! squares of the distances in each dimension.

use num_traits::Float;
use simd_euclidean::Vectorized;

/// Returns the squared euclidean distance between two points. When you only
/// need to compare distances, rather than having the exact distance between
/// the points, this metric is benefitial because it avoids the expensive square
/// root computation.
///
/// # Examples
///
/// ```rust
/// use kdtree::distance::squared_euclidean;
///
/// assert!(0.0 == squared_euclidean(&[0.0, 0.0], &[0.0, 0.0]));
/// assert!(2.0 == squared_euclidean(&[0.0, 0.0], &[1.0, 1.0]));
/// assert!(1.0 == squared_euclidean(&[0.0, 0.0], &[1.0, 0.0]));
/// ```
///
/// # Panics
///
/// Only in debug mode, the length of the slices at input will be compared.
/// If they do not match, there will be a panic:
///
/// ```rust,should_panic
/// # use kdtree::distance::squared_euclidean;
/// // this is broken
/// let _ = squared_euclidean(&[0.0, 0.0], &[1.0, 0.0, 0.0]);
/// ```
pub fn squared_euclidean_f32(a: &[f32], b: &[f32]) -> f32 {
    debug_assert_eq!(a.len(), b.len());
    Vectorized::distance(a, b)
}

pub fn squared_euclidean_f64(a: &[f64], b: &[f64]) -> f64 {
    debug_assert_eq!(a.len(), b.len());
    Vectorized::distance(a, b)
}
