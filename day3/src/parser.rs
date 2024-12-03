use std::fmt::Debug;
use std::str::FromStr;

use crate::Operation;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of},
    combinator::{peek, recognize},
    multi::many1,
    IResult, Parser as _,
};

/// Parse an integer of type `T`.
fn integer<T>(input: &str) -> IResult<&str, T>
where
    <T as FromStr>::Err: Debug,
    T: FromStr,
{
    recognize(many1(one_of("0123456789")))
        .parse(input)
        .map(|(i, s)| (i, s.parse().expect("decimal")))
}

/// Parse [`Operation::Mul`].
pub(super) fn mul_op<T>(i: &str) -> IResult<&str, Operation<T>>
where
    <T as FromStr>::Err: Debug,
    T: FromStr + Copy,
{
    let (i, _) = tag("mul")(i)?;
    let (i, _) = tag("(")(i)?;
    let (i, left) = integer(i)?;
    let (i, _) = char(',')(i)?;
    let (i, right) = integer(i)?;
    let (i, _) = tag(")")(i)?;
    Ok((i, Operation::Mul { left, right }))
}

/// Parse [`Operation::Do`].
fn do_op<T>(i: &str) -> IResult<&str, Operation<T>> {
    let (i, _) = tag("do()")(i)?;
    Ok((i, Operation::Do))
}

/// Parse [`Operation::Dont`].
fn dont_op<T>(i: &str) -> IResult<&str, Operation<T>> {
    let (i, _) = tag("don't()")(i)?;
    Ok((i, Operation::Dont))
}

/// Parse any [`Operation`].
pub(super) fn any_op<T>(i: &str) -> IResult<&str, Operation<T>>
where
    <T as FromStr>::Err: Debug,
    T: Copy + FromStr,
{
    alt((mul_op::<T>, do_op, dont_op))(i)
}

/// Skip one character in the provided string and return the remainder.
/// Returns `None` if the string is empty.
fn skip_one(i: &str) -> Option<&str> {
    i.split_at_checked(1).map(|(_, i)| i)
}

/// Parse a string into a list of [`Operation`]s.
/// The provided `ops_parser` is used to peek forward and attempt to parse an [`Operation`].
/// This allows limiting the amount of supported operations.
pub(super) fn with_ops_parser<T, F>(mut input: &str, ops_parser: F) -> Vec<Operation<T>>
where
    <T as FromStr>::Err: Debug,
    T: Copy + FromStr + Debug,
    F: Fn(&str) -> IResult<&str, Operation<T>> + Copy,
{
    let mut ops = Vec::new();
    // Peek forward and parse operations.
    loop {
        // Attempt to peek the next operation.
        match peek(ops_parser)(input) {
            // Operation found and parsed successfully.
            // Add operation to list and continue from remainder of peek.
            Ok((peek_remainder, op)) => {
                ops.push(op);
                input = peek_remainder;
            }
            // Incomplete input.
            // There are no more operations until end of input.
            Err(nom::Err::Incomplete(_)) => return ops,
            // Invalid input, ignore.
            // This is ok, we are expecting noisy input!
            Err(_) => {}
        };
        // Skip one character and attempt to peek at next iteration.
        match skip_one(input) {
            Some(next_input) => input = next_input,
            // Unable to skip - end of input.
            None => return ops,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify [`mul_op`] parsing works.
    #[test]
    fn test_mul() {
        assert_eq!(
            any_op("mul(1,2)").unwrap().1,
            Operation::Mul { left: 1, right: 2 }
        );
    }

    /// Verify [`do_op`] parsing works.
    #[test]
    fn test_do() {
        assert_eq!(any_op::<u32>("do()").unwrap().1, Operation::Do);
    }

    /// Verify [`dont_op`] parsing works.
    #[test]
    fn test_dont() {
        assert_eq!(any_op::<u32>("don't()").unwrap().1, Operation::Dont);
    }
}
