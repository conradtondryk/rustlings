// enums13.rs
//
// Real-world scenario: You're building a notification system. You need to
// define enums and structs that work together to represent different types
// of notifications.
//
// Learning Resources:
// - The Rust Book - Structs:
//   https://doc.rust-lang.org/book/ch05-01-defining-structs.html
// - The Rust Book - Enums:
//   https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
// - Rust by Example - Structs:
//   https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
//
// Execute `rustlings hint enums13` or use the `hint` watch subcommand for a
// hint.

// TODO: Define a struct called `User` with fields:
// - id: u32
// - name: String
struct User {
    id: u32,
    name: String,
}

// TODO: Define an enum called `Notification` with these variants:
// - Message (struct variant with: from (User), content (String))
// - Like (struct variant with: from (User), post_id (u32))
// - Follow (tuple variant holding a User)
// - SystemAlert (tuple variant holding a String for the alert message)

enum Notification {
    Message { from: User, content: String },
    Like { from: User, post_id: u32 },
    Follow(User),
    SystemAlert(String),
}

fn display_notification(notification: Notification) {
    match notification {
        Notification::Message { from, content } => {
            println!("{} sent you a message: {}", from.name, content);
        }
        Notification::Like { from, post_id } => {
            println!("{} liked your post #{}", from.name, post_id);
        }
        Notification::Follow(user) => {
            println!("{} started following you!", user.name);
        }
        Notification::SystemAlert(message) => {
            println!("SYSTEM: {}", message);
        }
    }
}

fn main() {
    let alice = User {
        id: 1,
        name: String::from("Alice"),
    };

    let bob = User {
        id: 2,
        name: String::from("Bob"),
    };

    let notif1 = Notification::Message {
        from: alice,
        content: String::from("Hey! How are you?"),
    };

    let notif2 = Notification::Follow(bob);

    let notif3 = Notification::SystemAlert(String::from("Maintenance in 1 hour"));

    display_notification(notif1);
    display_notification(notif2);
    display_notification(notif3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User {
            id: 100,
            name: String::from("Test User"),
        };
        assert_eq!(user.id, 100);
        assert_eq!(user.name, "Test User");
    }
}
