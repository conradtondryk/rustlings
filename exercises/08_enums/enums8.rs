// enums8.rs
//
// Enums can be nested inside other enums! This exercise explores working
// with nested enums and more complex pattern matching.
//
// Execute `rustlings hint enums8` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug, PartialEq)]
pub enum Shape {
    Circle(f64),         // radius
    Rectangle(f64, f64), // width, height
    Triangle(f64, f64),  // base, height
}

#[derive(Debug, PartialEq)]
pub enum DrawCommand {
    Draw(Shape),
    Move { x: i32, y: i32 },
    Clear,
    SetColor(u8, u8, u8),
}

// TODO: Implement this function to calculate the area of a shape.
// Circle: π * r²  (use 3.14159 for π)
// Rectangle: width * height
// Triangle: (base * height) / 2
#[allow(clippy::approx_constant)]
fn calculate_area(shape: &Shape) -> f64 {
    use Shape::*;
    match shape {
        Circle(r) => 3.14159 * r.powf(2.0),
        Rectangle(w, h) => w * h,
        Triangle(b, h) => b * h / 2.0,
    }
}

// TODO: Implement this function to extract a Shape from a DrawCommand.
// Return Some(shape) if the command is Draw(shape), None otherwise.
fn extract_shape(command: DrawCommand) -> Option<Shape> {
    match command {
        DrawCommand::Draw(s) => Some(s),
        _ => None,
    }
}

// TODO: Implement this function to process a draw command.
// If it's Draw(shape), return the area of the shape.
// For all other commands, return 0.0.
fn process_command(command: DrawCommand) -> f64 {
    use Shape::*;
    match command {
        DrawCommand::Draw(s) => match s {
            Circle(r) => calculate_area(&Circle(r)),
            Rectangle(w, h) => calculate_area(&Rectangle(w, h)),
            Triangle(b, h) => calculate_area(&Triangle(b, h)),
        },
        _ => 0.0,
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_area() {
        let circle = Shape::Circle(2.0);
        let rectangle = Shape::Rectangle(4.0, 5.0);
        let triangle = Shape::Triangle(6.0, 8.0);

        assert_eq!(calculate_area(&circle), 12.56636);
        assert_eq!(calculate_area(&rectangle), 20.0);
        assert_eq!(calculate_area(&triangle), 24.0);
    }

    #[test]
    fn test_extract_shape() {
        assert_eq!(
            extract_shape(DrawCommand::Draw(Shape::Circle(5.0))),
            Some(Shape::Circle(5.0))
        );
        assert_eq!(extract_shape(DrawCommand::Clear), None);
        assert_eq!(extract_shape(DrawCommand::Move { x: 1, y: 2 }), None);
    }

    #[test]
    fn test_process_command() {
        assert_eq!(
            process_command(DrawCommand::Draw(Shape::Rectangle(5.0, 4.0))),
            20.0
        );
        assert_eq!(process_command(DrawCommand::Clear), 0.0);
    }
}
