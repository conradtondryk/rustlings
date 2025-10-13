// enums12.rs
//
// This exercise combines everything: creating enums with mixed variant types
// (unit, tuple, and struct variants all in one enum).
//
// Learning Resources:
// - The Rust Book - Defining an Enum:
//   https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
// - Rust by Example - Enums:
//   https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
// - The Rust Book - Pattern Matching:
//   https://doc.rust-lang.org/book/ch06-02-match.html
//
// Execute `rustlings hint enums12` or use the `hint` watch subcommand for a
// hint.

// TODO: Define an enum called `GameAction` with these variants:
// - Jump (no data - unit variant)
// - Move (struct variant with named fields: x (i32), y (i32))
// - Attack (tuple variant holding a String for weapon name and u32 for damage)
// - Quit (no data - unit variant)

enum GameAction {
    Jump,
    Move { x: i32, y: i32 },
    Attack(String, u32),
    Quit,
}

// TODO: Define an enum called `DatabaseOperation` with these variants:
// - Read (tuple variant holding a String for the table name)
// - Write (struct variant with: table (String), data (String))
// - Delete (tuple variant holding a String for table and a u32 for id)
// - Close (no data - unit variant)

enum DatabaseOperation {
    Read(String),
    Write { table: String, data: String },
    Delete(String, u32),
    Close,
}

fn execute_action(action: GameAction) {
    match action {
        GameAction::Jump => println!("Player jumped!"),
        GameAction::Move { x, y } => println!("Player moved to ({}, {})", x, y),
        GameAction::Attack(weapon, damage) => {
            println!("Player attacked with {} for {} damage!", weapon, damage)
        }
        GameAction::Quit => println!("Game over!"),
    }
}

fn main() {
    // These should all compile once you define the enums correctly
    let action1 = GameAction::Jump;
    let action2 = GameAction::Move { x: 10, y: 20 };
    let action3 = GameAction::Attack(String::from("Sword"), 50);
    let action4 = GameAction::Quit;

    execute_action(action1);
    execute_action(action2);
    execute_action(action3);
    execute_action(action4);

    let _db_op1 = DatabaseOperation::Read(String::from("users"));
    let _db_op2 = DatabaseOperation::Write {
        table: String::from("posts"),
        data: String::from("Hello World"),
    };
    let _db_op3 = DatabaseOperation::Delete(String::from("comments"), 42);
    let _db_op4 = DatabaseOperation::Close;
}
