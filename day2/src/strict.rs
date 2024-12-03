//! Part 1 implementation.
//!
//! A report is safe if:
//! - Levels are only either increasing or decreasing.
//! - Difference between adjacent levels is in `1..=3`.

use crate::{Level, LevelInt};
use abs_diff::AbsDiff as _;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools as _;
use num::cast::AsPrimitive;
use std::cmp::Ordering;

/// Count the number of reports that are safe.
pub(super) fn safe_reports_count<T>(reports: &[Vec<T>]) -> usize
where
    T: Level,
    LevelInt: AsPrimitive<T>,
{
    reports
        .iter()
        .filter(|report| report_safety(report))
        .count()
}

/// Check if a report is safe.
pub(crate) fn report_safety<T>(report: &[T]) -> bool
where
    T: Level,
    LevelInt: AsPrimitive<T>,
{
    increasing_or_decreasing(report.iter().cloned()) && adjacent_safety(report.iter().cloned())
}

/// Check that levels are either increasing or decreasing.
pub(crate) fn increasing_or_decreasing<T>(report: impl Iterator<Item = T>) -> bool
where
    T: Level,
{
    let result = report.tuple_windows().fold_while(None, |acc, (a, b)| {
        let ord = a.cmp(&b);
        if ord == Ordering::Equal {
            // Items must be increasing OR decreasing.
            return Done(None);
        }
        if let Some(prev_ord) = acc {
            if prev_ord != ord {
                // A sequence that has been increasing OR decreasing so far must never
                // change ordering.
                return Done(None);
            }
        }
        Continue(Some(ord))
    });

    match result {
        Done(_) => false,
        // assume Continue(None) aka empty report means failure
        Continue(ord_opt) => ord_opt.is_some(),
    }
}

/// Check that difference between adjacent levels is within range 1..=3.
pub(crate) fn adjacent_safety<T>(report: impl Iterator<Item = T>) -> bool
where
    T: Level,
    LevelInt: AsPrimitive<T>,
{
    report
        .tuple_windows()
        .all(|(a, b)| (1.as_()..=3.as_()).contains(&a.abs_diff(b)))
}

#[cfg(test)]
mod tests {
    use crate::{parse_reports, strict, tests::generate_input};

    #[bench]
    fn bench_safe_reports_count(c: &mut test::Bencher) {
        let input = generate_input(1000);
        let reports = crate::parse_reports(&input);
        c.iter(|| strict::safe_reports_count(&reports));
    }

    /// Verify that input is either increasing or decreasing.
    #[test]
    fn ord() {
        let input = "1 2 3";
        let reports = parse_reports(input);
        assert!(strict::increasing_or_decreasing(reports[0].iter().cloned()));
    }

    /// Verify that input is not only increasing or only decreasing.
    #[test]
    fn ord2() {
        let input = "1 2 2";
        let reports = parse_reports(input);
        assert!(!strict::increasing_or_decreasing(
            reports[0].iter().cloned()
        ));
    }
}
