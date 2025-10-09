// enums4.rs
//
// This exercise introduces you to the Option<T> enum, one of Rust's most
// useful enums. Option is used to represent a value that might be present
// (Some) or absent (None).
//
// Execute `rustlings hint enums4` or use the `hint` watch subcommand for a
// hint.

fn main() {
    // Don't change this array!
    let numbers = vec![Some(5), None, Some(10), Some(15), None, Some(20)];

    // TODO: Use pattern matching to sum only the Some values.
    // The sum should be 50 (5 + 10 + 15 + 20).
    let sum = 0;

    println!("The sum is: {}", sum);
    assert_eq!(sum, 50);
}
