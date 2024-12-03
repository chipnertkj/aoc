//! Extension trait, [`AbsDiff`], for absolute difference between two generic integers.

/// Extension trait for absolute difference between two generic integers.
/// Provides method [`abs_diff`](AbsDiff::abs_diff).
pub trait AbsDiff {
    fn abs_diff(self, other: Self) -> Self;
}

impl<T> AbsDiff for T
where
    T: num::Integer,
{
    /// Compute the absolute difference between two integers.
    fn abs_diff(self, other: Self) -> Self {
        if self > other {
            self - other
        } else {
            other - self
        }
    }
}
