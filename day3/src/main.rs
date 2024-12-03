#![feature(test)]
#![feature(iter_array_chunks)]

use std::{iter::Sum, ops::Mul};

mod parser;

// Benchmarking utility.
extern crate test;

/// Solve both parts and print results to standard output.
fn main() {
    let input = input::stdin_file();
    let instructions = parser::with_ops_parser(&input, parser::mul_op);
    let output: u32 = execute(&instructions);
    println!("execution result (only `mul`): {output}");
    let instructions = parser::with_ops_parser(&input, parser::any_op);
    let output: u32 = execute(&instructions);
    println!("execution result (all operations): {output}");
}

/// Instruction to execute.
#[derive(Debug, PartialEq, Clone, Copy)]
enum Operation<T> {
    Mul { left: T, right: T },
    Do,
    Dont,
}

impl<T> Operation<T>
where
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    /// Execute the operation. This may affect the state provided to the function.
    /// Returns the result of the operation if applicable.
    fn execute(&self, enabled_state: &mut bool) -> Option<T> {
        match self {
            Operation::Mul { left, right } => enabled_state.then(|| left * right),
            Operation::Do => {
                *enabled_state = true;
                None
            }
            Operation::Dont => {
                *enabled_state = false;
                None
            }
        }
    }
}

/// Execute a sequence of instructions.
/// Returns the result of the execution.
fn execute<T>(instructions: &[Operation<T>]) -> T
where
    for<'a> &'a T: Mul<&'a T, Output = T>,
    T: Sum,
{
    // This is the retained state of the program that is formed
    // by our instruction list.
    let mut enabled = true;
    instructions
        // This has to be executed sequentially!
        // `Do`/`Dont` ops can change state, which affect `Mul` ops.
        .iter()
        // Fortunately, a `&mut` stops us from mistakenly sharing state
        // between threads anyway. :)
        .filter_map(|i| i.execute(&mut enabled))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{execute, parser};

    /// Test part 1 on sample input.
    #[test]
    fn part1() {
        let instructions =
            parser::with_ops_parser(include_str!("../sample_input_mul.txt"), parser::mul_op);
        assert_eq!(161u32, execute(&instructions));
    }

    /// Test part 2 on sample input.
    #[test]
    fn part2() {
        let instructions =
            parser::with_ops_parser(include_str!("../sample_input_any.txt"), parser::any_op);
        assert_eq!(48u32, execute(&instructions));
    }
}
