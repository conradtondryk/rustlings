// enums10.rs
//
// This exercise focuses on CREATING enum definitions from scratch.
// You'll practice defining enums with different types of variants.
//
// Execute `rustlings hint enums10` or use the `hint` watch subcommand for a
// hint.

// TODO: Define an enum called `Pet` with the following variants:
// - Dog (no data)
// - Cat (no data)
// - Bird (no data)
// - Fish (no data)

// TODO: Define an enum called `IpAddress` with the following variants:
// - V4 - holds a single String
// - V6 - holds a single String

// TODO: Define an enum called `HttpStatus` with these variants:
// - Ok - holds a u16 (status code)
// - NotFound - holds a String (error message)
// - ServerError - holds a u16 (status code) and a String (error message)

fn describe_pet(pet: Pet) -> String {
    match pet {
        Pet::Dog => String::from("Woof!"),
        Pet::Cat => String::from("Meow!"),
        Pet::Bird => String::from("Tweet!"),
        Pet::Fish => String::from("Blub!"),
    }
}

fn main() {
    let my_pet = Pet::Dog;
    println!("{}", describe_pet(my_pet));

    let ipv4 = IpAddress::V4(String::from("127.0.0.1"));
    let ipv6 = IpAddress::V6(String::from("::1"));

    let ok_status = HttpStatus::Ok(200);
    let not_found = HttpStatus::NotFound(String::from("Page not found"));
    let server_error = HttpStatus::ServerError(500, String::from("Internal error"));
}
