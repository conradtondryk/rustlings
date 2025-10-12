// enums11.rs
//
// This exercise teaches you how to create enums with struct-like variants.
// These are useful when you want named fields in your enum variants.
//
// Execute `rustlings hint enums11` or use the `hint` watch subcommand for a
// hint.

// TODO: Define an enum called `MouseEvent` with the following variants:
// - Click - with named fields: x (i32), y (i32), button (String)
// - Scroll - with named fields: delta_x (i32), delta_y (i32)
// - Move - with named fields: x (i32), y (i32)

enum MouseEvent {
    Click { x: i32, y: i32, button: String },
    Scroll { delta_x: i32, delta_y: i32 },
    Move { x: i32, y: i32 },
}

enum PaymentMethod {
    Cash {
        amount: f64,
    },
    CreditCard {
        number: String,
        cvv: u16,
        amount: f64,
    },
    Bitcoin {
        wallet_address: String,
    },
}

// TODO: Define an enum called `PaymentMethod` with these variants:
// - Cash - with a named field: amount (f64)
// - CreditCard - with named fields: number (String), cvv (u16), amount (f64)
// - Bitcoin - with a named field: wallet_address (String)

fn process_payment(payment: PaymentMethod) -> String {
    match payment {
        PaymentMethod::Cash { amount } => {
            format!("Paid ${:.2} in cash", amount)
        }
        PaymentMethod::CreditCard {
            number,
            cvv,
            amount,
        } => {
            format!(
                "Charged ${:.2} to card ending in {}",
                amount,
                &number[number.len() - 4..]
            )
        }
        PaymentMethod::Bitcoin { wallet_address } => {
            format!("Paid with Bitcoin to {}", wallet_address)
        }
    }
}

fn main() {
    let click = MouseEvent::Click {
        x: 100,
        y: 200,
        button: String::from("left"),
    };

    let payment = PaymentMethod::Cash { amount: 50.0 };
    println!("{}", process_payment(payment));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cash_payment() {
        let payment = PaymentMethod::Cash { amount: 25.50 };
        assert_eq!(process_payment(payment), "Paid $25.50 in cash");
    }
}
