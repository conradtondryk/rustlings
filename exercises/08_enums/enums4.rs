// enums4.rs
//
// This exercise introduces you to the Option<T> enum, one of Rust's most
// useful enums. Option is used to represent a value that might be present
// (Some) or absent (None).
//
// Learning Resources:
// - The Rust Book - Option:
//   https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values
// - The Rust Book - Pattern Matching:
//   https://doc.rust-lang.org/book/ch06-02-match.html
// - Rust by Example - Option:
//   https://doc.rust-lang.org/rust-by-example/std/option.html
//
// Execute `rustlings hint enums4` or use the `hint` watch subcommand for a
// hint.

fn main() {
    // Don't change this array!
    let numbers = vec![Some(5), None, Some(10), Some(15), None, Some(20)];

    // TODO: Use pattern matching to sum only the Some values.
    // The sum should be 50 (5 + 10 + 15 + 20).
    let sum = numbers.iter().fold(0, |acc, item| {
        if let Some(n) = item {
            acc + n
        } else {
            acc
        }
        acc
    });

    println!("The sum is: {}", sum);
    assert_eq!(sum, 50);
}
