#![feature(test)]

// Benchmarking utility.
extern crate test;

pub(crate) mod transform;
mod x_mas;
mod xmas;

/// Calculate width and height of input.
fn input_dimensions(input: &str) -> usize {
    let width = input.lines().next().expect("input must not be empty").len();
    let height = input.lines().count();
    assert_eq!(width, height);
    width // At this point `width == height`.
}

/// Solve both parts and print results to standard output.
fn main() {
    let input = input::stdin_file();
    let input = input.to_lowercase();
    let dimensions = input_dimensions(&input);
    let xmas_count = xmas::word_search_xmas_count(&input, dimensions);
    println!("instances of `xmas` found: {xmas_count}");
}

#[cfg(test)]
mod tests {
    use crate::{input_dimensions, transform, x_mas, xmas};

    /// Assert conversion to vertical axis works.
    #[test]
    fn vertical_axis() {
        let input = include_str!("../test_input.txt").to_lowercase();
        let expected = ["147", "258", "369"];
        let dimensions = input_dimensions(&input);
        let axis = transform::vertical(&input, dimensions);
        assert!(axis.iter().all(|d| expected.contains(&d.as_str())));
    }

    /// Assert conversion to falling diagonal axis works.
    #[test]
    fn falling_diagonal_axis() {
        let input = include_str!("../test_input.txt").to_lowercase();
        let expected = ["7", "48", "159", "26", "3"];
        let dimensions = input_dimensions(&input);
        let axis = transform::falling_diagonal(&input, dimensions);
        assert!(axis.iter().all(|d| expected.contains(&d.as_str())));
    }

    /// Assert conversion to rising diagonal axis works.
    #[test]
    fn rising_diagonal_axis() {
        let input = include_str!("../test_input.txt").to_lowercase();
        let expected = ["1", "42", "753", "86", "9"];
        let dimensions = input_dimensions(&input);
        let axis = transform::rising_diagonal(&input, dimensions);
        assert!(axis.iter().all(|d| expected.contains(&d.as_str())));
    }

    /// Test part 1 on sample input.
    #[test]
    fn part1() {
        let input = include_str!("../sample_input.txt").to_lowercase();
        let dimensions = input_dimensions(&input);
        assert_eq!(18, xmas::word_search_xmas_count(&input, dimensions));
    }

    /// Test part 2 on sample input.
    #[test]
    fn part2() {
        let input = include_str!("../sample_input.txt").to_lowercase();
        let dimensions = input_dimensions(&input);
        assert_eq!(9, x_mas::word_search_x_mas_count(&input, dimensions));
    }
}
