// enums17.rs
//
// Final boss: Implement a calculator that builds an expression tree using enums.
// This combines enums, recursion, pattern matching, and associated functions.
//
// Execute `rustlings hint enums17` or use the `hint` watch subcommand for a
// hint.

// TODO: Define an enum called `Expr` (short for Expression) with these variants:
// - Value(i32) - a simple number
// - Add(Box<Expr>, Box<Expr>) - addition of two expressions
// - Subtract(Box<Expr>, Box<Expr>) - subtraction
// - Multiply(Box<Expr>, Box<Expr>) - multiplication
// - Divide(Box<Expr>, Box<Expr>) - division
//
// Note: We use Box<Expr> because the enum is recursive!

// TODO: Implement these methods on Expr:
//
// Associated functions:
// - value(n: i32) -> Self
//   Creates a Value variant
//
// - add(left: Expr, right: Expr) -> Self
//   Creates an Add variant (remember to Box the expressions!)
//
// - subtract(left: Expr, right: Expr) -> Self
//   Creates a Subtract variant
//
// - multiply(left: Expr, right: Expr) -> Self
//   Creates a Multiply variant
//
// - divide(left: Expr, right: Expr) -> Self
//   Creates a Divide variant
//
// Methods:
// - eval(&self) -> Option<i32>
//   Evaluates the expression tree and returns the result.
//   Returns None if division by zero occurs.
//   This requires recursive evaluation!

fn main() {
    // Build expression: (5 + 3) * 2
    let expr = Expr::multiply(
        Expr::add(Expr::value(5), Expr::value(3)),
        Expr::value(2),
    );

    println!("(5 + 3) * 2 = {:?}", expr.eval());

    // Build expression: 10 / (5 - 5) - this should return None due to division by zero
    let bad_expr = Expr::divide(
        Expr::value(10),
        Expr::subtract(Expr::value(5), Expr::value(5)),
    );

    println!("10 / (5 - 5) = {:?}", bad_expr.eval());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_value() {
        let expr = Expr::value(42);
        assert_eq!(expr.eval(), Some(42));
    }

    #[test]
    fn test_simple_addition() {
        let expr = Expr::add(Expr::value(5), Expr::value(3));
        assert_eq!(expr.eval(), Some(8));
    }

    #[test]
    fn test_complex_expression() {
        // (10 + 5) * (8 - 3) = 15 * 5 = 75
        let expr = Expr::multiply(
            Expr::add(Expr::value(10), Expr::value(5)),
            Expr::subtract(Expr::value(8), Expr::value(3)),
        );
        assert_eq!(expr.eval(), Some(75));
    }

    #[test]
    fn test_division() {
        // 20 / 4 = 5
        let expr = Expr::divide(Expr::value(20), Expr::value(4));
        assert_eq!(expr.eval(), Some(5));
    }

    #[test]
    fn test_division_by_zero() {
        // 10 / 0 should return None
        let expr = Expr::divide(Expr::value(10), Expr::value(0));
        assert_eq!(expr.eval(), None);
    }

    #[test]
    fn test_nested_division_by_zero() {
        // 10 / (5 - 5) = 10 / 0 should return None
        let expr = Expr::divide(
            Expr::value(10),
            Expr::subtract(Expr::value(5), Expr::value(5)),
        );
        assert_eq!(expr.eval(), None);
    }

    #[test]
    fn test_very_complex_expression() {
        // ((100 - 50) / 5) + (3 * 4) = (50 / 5) + 12 = 10 + 12 = 22
        let expr = Expr::add(
            Expr::divide(
                Expr::subtract(Expr::value(100), Expr::value(50)),
                Expr::value(5),
            ),
            Expr::multiply(Expr::value(3), Expr::value(4)),
        );
        assert_eq!(expr.eval(), Some(22));
    }
}
