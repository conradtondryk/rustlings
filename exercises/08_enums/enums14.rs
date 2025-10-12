// enums14.rs
//
// This exercise combines enum definition with associated functions and methods.
// You'll build a temperature system that can work with different units.
//
// Execute `rustlings hint enums14` or use the `hint` watch subcommand for a
// hint.

// TODO: Define an enum called `Temperature` with these variants:
// - Celsius(f64)
// - Fahrenheit(f64)
// - Kelvin(f64)

// TODO: Implement these associated functions and methods on Temperature:
//
// Associated functions (called with Temperature::function_name):
// - new_celsius(temp: f64) -> Self
// - new_fahrenheit(temp: f64) -> Self
// - new_kelvin(temp: f64) -> Self
//
// Methods (called with instance.method_name()):
// - to_celsius(&self) -> f64
//   Returns the temperature in Celsius regardless of original unit
//   Formulas:
//   - F to C: (F - 32) * 5/9
//   - K to C: K - 273.15
//
// - to_fahrenheit(&self) -> f64
//   Returns the temperature in Fahrenheit
//   Formulas:
//   - C to F: C * 9/5 + 32
//   - K to F: (K - 273.15) * 9/5 + 32
//
// - is_freezing(&self) -> bool
//   Returns true if temperature is at or below 0°C

fn main() {
    let temp1 = Temperature::new_celsius(25.0);
    let temp2 = Temperature::new_fahrenheit(77.0);
    let temp3 = Temperature::new_kelvin(273.15);

    println!("Temp1 in F: {:.1}°F", temp1.to_fahrenheit());
    println!("Temp2 in C: {:.1}°C", temp2.to_celsius());
    println!("Is temp3 freezing? {}", temp3.is_freezing());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        let temp = Temperature::new_celsius(0.0);
        assert!((temp.to_fahrenheit() - 32.0).abs() < 0.01);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        let temp = Temperature::new_fahrenheit(32.0);
        assert!((temp.to_celsius() - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_kelvin_to_celsius() {
        let temp = Temperature::new_kelvin(273.15);
        assert!((temp.to_celsius() - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_is_freezing() {
        let temp1 = Temperature::new_celsius(-5.0);
        let temp2 = Temperature::new_celsius(5.0);
        let temp3 = Temperature::new_celsius(0.0);

        assert!(temp1.is_freezing());
        assert!(!temp2.is_freezing());
        assert!(temp3.is_freezing());
    }
}
