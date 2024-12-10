//! Part 2 implementation.

use crate::transform;

/// Find all occurences of `X-MAS` (two `MAS` in the shape of an `X`).
///
/// # Naming convention
/// Very big difference... See [`word_search_xmas_count`](crate::xmas::word_search_xmas_count)
/// and <https://adventofcode.com/2024/day/4> (part 2).
pub(super) fn word_search_x_mas_count(input: &str, dimensions: usize) -> u32 {
    // Build axes from input.
    let horizontal = transform::horizontal(input);
    let vertical = transform::vertical(input, dimensions);
    let diagonal_falling = transform::falling_diagonal(input, dimensions);
    let diagonal_rising = transform::rising_diagonal(input, dimensions);

    todo!()
}
