// enums5.rs
//
// This exercise will teach you about using `if let` as a more concise
// alternative to `match` when you only care about one specific pattern.
//
// Learning Resources:
// - The Rust Book - if let:
//   https://doc.rust-lang.org/book/ch06-03-if-let.html
// - Rust by Example - if let:
//   https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
//
// Execute `rustlings hint enums5` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect_event(event: WebEvent) {
    // TODO: Use `if let` to check if the event is a Click event.
    // If it is, print "Clicked at x={}, y={}".
    // For all other events, print "Other event: {:?}".
    if let WebEvent::Click { x, y } = event {
        println!("Clicked at x={}, y={}", x, y)
    } else {
        println!("Other event: {:?}", event)
    }
}

fn extract_key(event: WebEvent) -> Option<char> {
    // TODO: Use `if let` to extract and return the char from a KeyPress event.
    // Return None for all other events.
    if let WebEvent::KeyPress(char) = event {
        Some(char)
    } else {
        None
    }
}

fn main() {
    let events = vec![
        WebEvent::PageLoad,
        WebEvent::KeyPress('x'),
        WebEvent::Click { x: 20, y: 80 },
        WebEvent::Paste(String::from("test")),
        WebEvent::KeyPress('a'),
    ];

    for event in events {
        inspect_event(event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_key() {
        assert_eq!(extract_key(WebEvent::KeyPress('a')), Some('a'));
        assert_eq!(extract_key(WebEvent::PageLoad), None);
        assert_eq!(extract_key(WebEvent::Click { x: 1, y: 2 }), None);
    }
}
