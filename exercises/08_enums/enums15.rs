// enums15.rs
//
// Build a state machine for a traffic light system using enums with methods.
// This is a common pattern in Rust for modeling state transitions.
//
// Learning Resources:
// - The Rust Book - Enums:
//   https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
// - The Rust Book - Method Syntax:
//   https://doc.rust-lang.org/book/ch05-03-method-syntax.html
// - Rust by Example - Enums:
//   https://doc.rust-lang.org/rust-by-example/custom_types/enum.html
// - State Machine Pattern in Rust:
//   https://hoverbear.org/blog/rust-state-machine-pattern/
//
// Execute `rustlings hint enums15` or use the `hint` watch subcommand for a
// hint.

// TODO: Define an enum called `TrafficLightState` with these variants:
// - Red { time_remaining: u32 }
// - Yellow { time_remaining: u32 }
// - Green { time_remaining: u32 }

// TODO: Implement these methods on TrafficLightState:
//
// - new() -> Self
//   Creates a new traffic light starting in Red state with 60 seconds
//
// - tick(&mut self)
//   Decrements time_remaining by 1. If it reaches 0, transitions to next state:
//   Red (60s) -> Green (90s) -> Yellow (10s) -> Red (60s)
//
// - can_cross(&self) -> bool
//   Returns true only if the light is Red (pedestrians can cross)
//
// - time_remaining(&self) -> u32
//   Returns the time remaining in the current state
//
// - color_name(&self) -> &str
//   Returns "Red", "Yellow", or "Green"

fn main() {
    let mut light = TrafficLightState::new();

    println!("Light is {} with {}s remaining", light.color_name(), light.time_remaining());
    println!("Can cross? {}", light.can_cross());

    // Simulate 61 seconds
    for _ in 0..61 {
        light.tick();
    }

    println!("After 61s: Light is {} with {}s remaining",
             light.color_name(), light.time_remaining());
    println!("Can cross? {}", light.can_cross());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_starts_red() {
        let light = TrafficLightState::new();
        assert_eq!(light.color_name(), "Red");
        assert_eq!(light.time_remaining(), 60);
    }

    #[test]
    fn test_red_to_green_transition() {
        let mut light = TrafficLightState::new();

        // Tick through all 60 seconds of red
        for _ in 0..60 {
            light.tick();
        }

        assert_eq!(light.color_name(), "Green");
        assert_eq!(light.time_remaining(), 90);
    }

    #[test]
    fn test_full_cycle() {
        let mut light = TrafficLightState::new();

        // Red: 60s
        for _ in 0..60 {
            light.tick();
        }
        assert_eq!(light.color_name(), "Green");

        // Green: 90s
        for _ in 0..90 {
            light.tick();
        }
        assert_eq!(light.color_name(), "Yellow");

        // Yellow: 10s
        for _ in 0..10 {
            light.tick();
        }
        assert_eq!(light.color_name(), "Red");
        assert_eq!(light.time_remaining(), 60);
    }

    #[test]
    fn test_can_cross() {
        let mut light = TrafficLightState::new();
        assert!(light.can_cross()); // Red

        for _ in 0..60 {
            light.tick();
        }
        assert!(!light.can_cross()); // Green

        for _ in 0..90 {
            light.tick();
        }
        assert!(!light.can_cross()); // Yellow
    }
}
