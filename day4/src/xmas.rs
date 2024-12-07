//! Part 1 implementation.

use crate::transform;

/// Parser for the tag `xmas`.
fn xmas(i: &str) -> nom::IResult<&str, ()> {
    let (i, _) = nom::bytes::complete::tag("xmas")(i)?;
    Ok((i, ()))
}

/// Parser for the tag `samx`, or `xmas` reversed.
fn samx(i: &str) -> nom::IResult<&str, ()> {
    let (i, _) = nom::bytes::complete::tag("samx")(i)?;
    Ok((i, ()))
}

/// Count all occurrences of `xmas` in input along the horizontal, vertical and diagonal axes.
/// Directionality of the word is irrelevant - both `xmas` and `samx` are counted.
///
/// `dimensions` must match input height (lines/rows) and width (chars/columns).
pub(super) fn word_search_xmas_count(input: &str, dimensions: usize) -> u32 {
    // Build axes from input.
    let horizontal = transform::horizontal(input);
    let vertical = transform::vertical(input, dimensions);
    let diagonal_falling = transform::falling_diagonal(input, dimensions);
    let diagonal_rising = transform::rising_diagonal(input, dimensions);
    // Count xmas words in each axis.
    [
        count_xmas_and_samx_in_axis(horizontal),
        count_xmas_and_samx_in_axis(vertical),
        count_xmas_and_samx_in_axis(diagonal_falling),
        count_xmas_and_samx_in_axis(diagonal_rising),
    ]
    .into_iter()
    .sum()
}

/// Count all xmas occurrences in an axis.
fn count_xmas_and_samx_in_axis(lines: impl IntoIterator<Item = impl AsRef<str>>) -> u32 {
    lines
        .into_iter()
        .map(|line| count_occurrences_in_line(&line, xmas) + count_occurrences_in_line(line, samx))
        .sum::<u32>()
}

/// Skip one character in the provided string and return the remainder.
/// Returns `None` if the string is empty.
fn skip_one(i: &str) -> Option<&str> {
    i.split_at_checked(1).map(|(_, i)| i)
}

// TODO: consider extracting this to a utility crate?
// Same pattern used in `parser.rs` in day 3.
// Same with `skip_one`.
/// Count occurrences of a parser in a line.
/// Peeks ahead to see if the parser matches the current position.
fn count_occurrences_in_line<T>(
    line: impl AsRef<str>,
    parser: fn(&str) -> nom::IResult<&str, T>,
) -> u32 {
    let mut line = line.as_ref();
    let mut count = 0;
    // Peek forward and parse operations.
    loop {
        // Attempt to peek the next operation.
        if let Ok((remainder, _)) = nom::combinator::peek(parser)(line) {
            line = remainder;
            count += 1;
        }
        // Skip one character and attempt to peek at next iteration.
        match skip_one(line) {
            Some(next_input) => line = next_input,
            None => {
                // Unable to skip - end of input.
                return count;
            }
        }
    }
}
