#![feature(test)]

// Benchmarking utility.
extern crate test;

use num::Integer;

pub(crate) mod strict;
mod tolerant;

/// Functions that operate on reports require levels to satisfy this trait bound.
pub(crate) trait Level: Integer + Copy + 'static {}
impl<T> Level for T where T: Integer + Copy + 'static {}

/// Integer type levels are stored/parsed as.
pub(crate) type LevelInt = u32;

/// Solve both parts and print results to standard output.
fn main() {
    let input = input::stdin_file();
    let reports = parse_reports(&input);
    let count_strict = strict::safe_reports_count(&reports);
    let count_tolerant = tolerant::safe_reports_count(&reports);
    println!("safe reports (strict): {count_strict}");
    println!("safe reports (tolerant): {count_tolerant}");
}

/// Parse input into reports.
fn parse_reports(input: &str) -> Vec<Vec<LevelInt>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::{parse_reports, strict, tolerant};
    use itertools::Itertools;
    use rand::Rng;

    pub(crate) fn generate_input(reports: usize) -> String {
        let rand_level = || rand::thread_rng().gen_range(0..=110);
        let rand_level_shift = || rand::thread_rng().gen_range(-5..=5i32);
        let rand_report_len = || rand::thread_rng().gen_range(4..=8);
        let reports = (0..reports)
            .map(|_| {
                let len = rand_report_len();
                let shifts = (0..len)
                    .into_iter()
                    .map(|_| rand_level_shift())
                    .collect_vec();
                let mut level = shifts
                    .iter()
                    .fold(rand_level(), |acc, shift| acc + shift.abs());
                (0..len)
                    .into_iter()
                    .map(|i| {
                        let shift = shifts[i];
                        let old_level = level;
                        let new_level = level as i32 + shift;
                        level = new_level;
                        old_level
                    })
                    .collect_vec()
            })
            .collect_vec();
        let mut input = String::new();
        reports
            .into_iter()
            .for_each(|report| input.push_str(&format!("{}\n", report.iter().join(" "))));
        input
    }

    #[bench]
    fn bench_read_reports(c: &mut test::Bencher) {
        let input = generate_input(1000);
        c.iter(|| parse_reports(&input));
    }

    /// Test part 1 on sample input.
    #[test]
    fn part1() {
        let reports = parse_reports(include_str!("../sample_input.txt"));
        let output = strict::safe_reports_count(&reports);
        assert_eq!(output, 2);
    }

    /// Test part 2 on sample input.
    #[test]
    fn part2() {
        let reports = parse_reports(include_str!("../sample_input.txt"));
        let output = tolerant::safe_reports_count(&reports);
        assert_eq!(output, 4);
    }
}
