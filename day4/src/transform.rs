//! Functions to convert the input into different linear spaces.
//! These spaces are used to search for words efficiently at the cost
//! of some additional memory allocation.

/// Convert input to the horizontal axis (in lines/rows).
pub(crate) fn horizontal(input: &str) -> impl Iterator<Item = impl AsRef<str> + '_> {
    input.lines()
}

/// Convert input to the vertical axis (in columns).
pub(crate) fn vertical(input: &str, dimensions: usize) -> Vec<String> {
    let mut columns = Vec::with_capacity(dimensions);
    (0..dimensions).for_each(|_| columns.push(String::with_capacity(dimensions)));
    columns
        .iter_mut()
        .enumerate()
        .for_each(|(line_ix, column)| {
            input.lines().for_each(|line| {
                column.push(line.chars().nth(line_ix).expect("lines must not be empty"));
            });
        });
    columns
}

/// Calculate the number of diagonals in a square grid.
fn diagonals_count(dimensions: usize) -> usize {
    dimensions * 2 - 1
}

/// Calculate lengths of each diagonal in a square grid.
fn diagonal_lenghts<T>(dimensions: usize) -> impl Iterator<Item = T>
where
    T: num::Integer + std::ops::AddAssign + std::ops::SubAssign + Copy,
{
    let mut acc = T::zero();
    (0..diagonals_count(dimensions)).map(move |ix| {
        if ix >= dimensions {
            acc -= T::one();
        } else {
            acc += T::one();
        }
        acc
    })
}

/// Convert input to the falling diagonal axis.
pub(crate) fn falling_diagonal(input: &str, dimensions: usize) -> Vec<String> {
    // Calculate diagonal lengths and count.
    let diagonals_count = diagonals_count(dimensions);
    let diagonal_lenghts: Vec<_> = diagonal_lenghts(dimensions).collect();
    // Initialize diagonals.
    let mut diagonals = Vec::with_capacity(diagonals_count);
    (0..diagonals_count).for_each(|n| diagonals.push(String::with_capacity(diagonal_lenghts[n])));
    // Fill diagonals.
    diagonals
        .iter_mut()
        .enumerate()
        .for_each(|(diagonal_ix, diagonal)| {
            let len = diagonal_lenghts[diagonal_ix];
            let mut x = diagonal_ix.max(dimensions - 1) + 1 - dimensions + 1;
            let mut y = diagonal_ix.min(dimensions - 1) + 1;
            (0..len).for_each(|_| {
                let char = input
                    .lines()
                    .nth(dimensions - y)
                    .expect("y out of bounds")
                    .chars()
                    .nth(x - 1)
                    .expect("x out of bounds");
                diagonal.push(char);
                x += 1;
                y -= 1;
            });
        });
    diagonals
}

/// Convert input to the rising diagonal axis.
pub(crate) fn rising_diagonal(input: &str, dimensions: usize) -> Vec<String> {
    // Calculate diagonal lengths and count.
    let diagonals_count = diagonals_count(dimensions);
    let diagonal_lenghts: Vec<_> = diagonal_lenghts(dimensions).collect();
    // Initialize diagonals.
    let mut diagonals = Vec::with_capacity(diagonals_count);
    (0..diagonals_count).for_each(|n| diagonals.push(String::with_capacity(diagonal_lenghts[n])));
    // Fill diagonals.
    diagonals
        .iter_mut()
        .enumerate()
        .for_each(|(diagonal_ix, diagonal)| {
            let len = diagonal_lenghts[diagonal_ix];
            let mut x = diagonal_ix.max(dimensions - 1) + 1 - dimensions + 1;
            let mut y = diagonal_ix.min(dimensions - 1) + 1;
            (0..len).for_each(|_| {
                let char = input
                    .lines()
                    .nth(y - 1)
                    .expect("y out of bounds")
                    .chars()
                    .nth(x - 1)
                    .expect("x out of bounds");
                diagonal.push(char);
                x += 1;
                y -= 1;
            });
        });
    diagonals
}
