// structs4.rs
//
// This exercise will teach you how to define structs from scratch based on
// requirements. You'll practice choosing appropriate field types.
//
// Learning Resources:
// - The Rust Book - Defining Structs:
//   https://doc.rust-lang.org/book/ch05-01-defining-structs.html
// - Rust by Example - Structs:
//   https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
//
// Execute `rustlings hint structs4` or use the `hint` watch subcommand for a
// hint.

// TODO: Define a struct called `Book` that has the following fields:
// - title: a String
// - author: a String
// - pages: a u32
// - available: a bool

struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}

// TODO: Define a tuple struct called `RGB` that holds three u8 values
// representing red, green, and blue color components.

struct RGB(u8, u8, u8);

// TODO: Define a unit struct called `Marker` (these are useful for generic types)
struct Marker;

fn main() {
    // Don't modify this code - it should compile once you've defined the structs above
    let my_book = Book {
        title: String::from("The Rust Book"),
        author: String::from("Steve Klabnik"),
        pages: 552,
        available: true,
    };

    let white = RGB(255, 255, 255);
    let _marker = Marker;

    println!("Book: {} by {}", my_book.title, my_book.author);
    println!("RGB: ({}, {}, {})", white.0, white.1, white.2);
}
