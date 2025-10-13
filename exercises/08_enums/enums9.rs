// enums9.rs
//
// This final enum exercise combines multiple concepts: generic enums,
// the `?` operator for error propagation, and working with both Option and
// Result types together.
//
// Learning Resources:
// - The Rust Book - The ? Operator:
//   https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
// - The Rust Book - Result:
//   https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
// - Rust by Example - Error Handling:
//   https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/reenter_question_mark.html
//
// Execute `rustlings hint enums9` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug, PartialEq)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

// TODO: Implement this function to safely divide two numbers.
// Return Err(MathError::DivisionByZero) if b is 0.
fn safe_divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

// TODO: Implement this function to calculate the square root of a number.
// Return Err(MathError::NegativeSquareRoot) if n is negative.
// You can use (n as f64).sqrt() and convert back to i32.
fn safe_sqrt(n: i32) -> Result<i32, MathError> {
    if n >= 0 {
        Ok((n as f64).sqrt() as i32)
    } else {
        Err(MathError::NegativeSquareRoot)
    }
}

// TODO: Implement this function that uses the `?` operator to chain
// multiple operations together. The function should:
// 1. Divide `a` by `b`
// 2. Take the square root of the result
// 3. Return the final value
// Use the `?` operator to propagate errors from safe_divide and safe_sqrt.
fn divide_and_sqrt(a: i32, b: i32) -> Result<i32, MathError> {
    safe_sqrt(safe_divide(a, b)?)
}

// TODO: Implement this function to get the first element of a vector
// and add a value to it. Return None if the vector is empty.
fn add_to_first(vec: Vec<i32>, value: i32) -> Option<i32> {
    if vec.is_empty() {
        None
    } else {
        Some(vec.first().unwrap() + value)
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10, 2), Ok(5));
        assert_eq!(safe_divide(10, 0), Err(MathError::DivisionByZero));
    }

    #[test]
    fn test_safe_sqrt() {
        assert_eq!(safe_sqrt(16), Ok(4));
        assert_eq!(safe_sqrt(-1), Err(MathError::NegativeSquareRoot));
    }

    #[test]
    fn test_divide_and_sqrt() {
        assert_eq!(divide_and_sqrt(100, 4), Ok(5));
        assert_eq!(divide_and_sqrt(10, 0), Err(MathError::DivisionByZero));
        // Dividing -100 by 4 gives -25, which is negative
        assert_eq!(divide_and_sqrt(-100, 4), Err(MathError::NegativeSquareRoot));
    }

    #[test]
    fn test_add_to_first() {
        assert_eq!(add_to_first(vec![1, 2, 3], 10), Some(11));
        assert_eq!(add_to_first(vec![], 10), None);
    }
}
