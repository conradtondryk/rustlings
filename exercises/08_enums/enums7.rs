// enums7.rs
//
// This exercise introduces you to the Result<T, E> enum, which is used for
// error handling in Rust. A Result can be either Ok(value) or Err(error).
//
// Execute `rustlings hint enums7` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug, PartialEq)]
enum DivisionError {
    DivideByZero,
    NotDivisible,
}

// TODO: Implement this function to return Ok if b divides a evenly,
// Err(DivisionError::DivideByZero) if b is zero, and
// Err(DivisionError::NotDivisible) if a is not evenly divisible by b.
fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    todo!()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10, 2), Ok(5));
        assert_eq!(divide(20, 4), Ok(5));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(10, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(divide(10, 3), Err(DivisionError::NotDivisible));
    }
}
