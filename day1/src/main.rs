#![feature(test)]

// Benchmarking utility.
extern crate test;

use abs_diff::AbsDiff;
use itertools::Itertools as _;
use num::{cast::AsPrimitive, Integer};
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};
use std::{fmt::Debug, iter::Sum, str::FromStr};

/// Functions that operate on the lists require items to satisfy this trait bound.
pub(crate) trait Item: Integer + Copy + 'static {}
impl<T> Item for T where T: Integer + Copy + 'static {}

/// Integer type list items are stored/parsed as.
type ItemInt = u32;

/// Solve both parts and print results to standard output.
fn main() {
    let input = input::stdin_file();
    let (mut left, mut right) = parse_lists::<ItemInt>(&input);
    let similarity_score = similarity_score(&left, &right);
    let list_distance = list_distance(&mut left, &mut right);
    println!("list distance: {list_distance}");
    println!("similarity score: {similarity_score}");
}

/// Parse input into two lists.
/// Ensure that the lists have the same length.
fn parse_lists<T>(input: &str) -> (Vec<T>, Vec<T>)
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut left = vec![];
    let mut right = vec![];
    input
        .split_ascii_whitespace()
        .map(|s| s.trim().parse().expect("items must be parsable to T"))
        .chunks(2)
        .into_iter()
        .for_each(|mut chunk| {
            left.push(chunk.next().expect("doesn't return empty chunks"));
            right.push(chunk.next().expect("lists must have same length"));
        });
    (left, right)
}

/// Compute the "list distance" between two lists.
/// Sorts both lists first.
fn list_distance<T>(left: &mut [T], right: &mut [T]) -> T
where
    T: Item,
    ItemInt: AsPrimitive<T>,
{
    left.sort_unstable();
    right.sort_unstable();
    left.iter_mut()
        .zip(right)
        .fold(0.as_(), |acc, (l, r)| acc + l.abs_diff(*r))
}

/// Compute the "similarity score" between two lists.
fn similarity_score<T>(left: &[T], right: &[T]) -> T
where
    T: Item + Sync + Send + Sum,
    usize: AsPrimitive<T>,
{
    // Compute bound - use parallelism to speed up.
    left.par_iter()
        .map(|l| right.iter().filter(|r| l == *r).count().as_() * *l)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{list_distance, parse_lists, similarity_score, ItemInt};
    use num::cast::AsPrimitive as _;

    pub(crate) fn generate_input(len: usize) -> String {
        let mut input = String::new();
        for _ in 0..len {
            let rand_item = || rand::random::<u16>();
            let (left, right) = (rand_item(), rand_item());
            input.push_str(&format!("{left} {right}\n"));
        }
        input
    }

    #[bench]
    fn bench_read_list(c: &mut test::Bencher) {
        let input = generate_input(1000);
        c.iter(|| parse_lists::<ItemInt>(&input));
    }

    #[bench]
    fn bench_list_distance(c: &mut test::Bencher) {
        let input = generate_input(1000);
        let (left, right) = parse_lists::<ItemInt>(&input);
        c.iter(move || {
            let mut left = left.clone();
            let mut right = right.clone();
            list_distance(&mut left, &mut right)
        });
    }

    #[bench]
    fn bench_similarity_score(c: &mut test::Bencher) {
        let input = generate_input(1000);
        let (left, right) = parse_lists::<ItemInt>(&input);
        c.iter(|| similarity_score(&left, &right));
    }

    /// Test part 1 on sample input.
    #[test]
    fn part1() {
        let (mut left, mut right) = parse_lists::<ItemInt>(include_str!("../sample_input.txt"));
        let output = list_distance(&mut left, &mut right);
        assert_eq!(output, 11.as_());
    }

    /// Test part 2 on sample input.
    #[test]
    fn part2() {
        let (left, right) = parse_lists::<ItemInt>(include_str!("../sample_input.txt"));
        let output = similarity_score(&left, &right);
        assert_eq!(output, 31.as_());
    }
}
