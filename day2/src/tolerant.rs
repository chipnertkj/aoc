//! Part 2 implementation.
//!
//! A report is safe if:
//! - Levels are only either increasing or decreasing.
//! - Difference between adjacent levels is in `1..=3`.
//! - The report would otherwise be safe according to the previous rules
//!   as long as at most one of the levels is removed from the report.

use crate::{strict, Level, LevelInt};
use num::cast::AsPrimitive;
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};

/// Count the number of reports that are safe.
pub(super) fn safe_reports_count<T>(reports: &[Vec<T>]) -> usize
where
    T: Level + Sync,
    LevelInt: AsPrimitive<T>,
{
    reports
        // Compute bound - use parallelism to speed up.
        .par_iter()
        .filter(|report| report_safety(report))
        .count()
}

/// Check if a report is safe.
/// If safety fails using strict rules, try skipping one level from the report and check again,
/// until all attempts fail.
fn report_safety<T>(report: &[T]) -> bool
where
    T: Level,
    LevelInt: AsPrimitive<T>,
{
    strict::report_safety(report) || (0..report.len()).any(|n| report_safety_skip_nth(report, n))
}

/// Check if a report is safe if the nth level (`n`) was excluded from the report.
fn report_safety_skip_nth<T>(report: &[T], n: usize) -> bool
where
    T: Level,
    LevelInt: AsPrimitive<T>,
{
    strict::increasing_or_decreasing(skip_nth_level(report.iter().cloned(), n))
        && strict::adjacent_safety(skip_nth_level(report.iter().cloned(), n))
}

/// Skip the nth level from the report.
///
/// Returns an iterator that omits the nth level from the report.
fn skip_nth_level<T>(report: impl Iterator<Item = T>, n: usize) -> impl Iterator<Item = T> {
    report
        .enumerate()
        .filter_map(move |(index, level)| if index == n { None } else { Some(level) })
}

#[cfg(test)]
mod tests {
    use crate::{parse_reports, tests::generate_input, tolerant};

    #[bench]
    fn bench_safe_reports_count(c: &mut test::Bencher) {
        let input = generate_input(1000);
        let reports = parse_reports(&input);
        c.iter(|| tolerant::safe_reports_count(&reports));
    }
}
