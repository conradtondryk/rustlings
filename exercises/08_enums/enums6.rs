// enums6.rs
//
// Enums can have methods implemented on them, just like structs!
// This exercise will have you implement methods to work with a traffic
// light system.
//
// Execute `rustlings hint enums6` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug, PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    // TODO: Implement a method that returns the duration in seconds
    // that each light should stay on.
    // Red: 60 seconds, Yellow: 10 seconds, Green: 90 seconds
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 10,
            TrafficLight::Green => 90,
        }
    }

    // TODO: Implement a method that returns the next traffic light state.
    // Red -> Green, Green -> Yellow, Yellow -> Red
    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
        }
    }

    // TODO: Implement a method that returns true if it's safe to cross
    // the street (only when the light is Red).
    fn can_cross(&self) -> bool {
        matches!(self, TrafficLight::Red)
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration() {
        assert_eq!(TrafficLight::Red.duration(), 60);
        assert_eq!(TrafficLight::Yellow.duration(), 10);
        assert_eq!(TrafficLight::Green.duration(), 90);
    }

    #[test]
    fn test_next() {
        assert_eq!(TrafficLight::Red.next(), TrafficLight::Green);
        assert_eq!(TrafficLight::Green.next(), TrafficLight::Yellow);
        assert_eq!(TrafficLight::Yellow.next(), TrafficLight::Red);
    }

    #[test]
    fn test_can_cross() {
        assert!(TrafficLight::Red.can_cross());
        assert!(!TrafficLight::Yellow.can_cross());
        assert!(!TrafficLight::Green.can_cross());
    }
}
