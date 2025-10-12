// structs5.rs
//
// Sometimes you need to store optional data in structs, or data that might
// fail to be created. This exercise teaches you how to use Option and Result
// in struct fields.
//
// Execute `rustlings hint structs5` or use the `hint` watch subcommand for a
// hint.

// TODO: Define a struct called `User` with the following fields:
// - username: String
// - email: String
// - age: Option<u32>  (age is optional - not everyone wants to share it!)
// - profile_picture: Option<String>  (users might not have uploaded one yet)

struct User {
    username: String,
    email: String,
    age: Option<u32>,
    profile_picture: Option<String>,
}

fn create_user(username: String, email: String) -> User {
    // TODO: Create and return a User with no age or profile picture
    User {
        username,
        email,
        age: None,
        profile_picture: None,
    }
}

fn main() {
    let user1 = create_user(String::from("rustacean"), String::from("rust@example.com"));

    // This should compile and print the username
    println!("Username: {}", user1.username);

    // TODO: Create a user2 with an age of 25 and no profile picture

    // TODO: Create a user3 with both an age of 30 and a profile picture URL
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user() {
        let user = create_user(String::from("test"), String::from("test@test.com"));
        assert_eq!(user.username, "test");
        assert_eq!(user.age, None);
        assert_eq!(user.profile_picture, None);
    }
}
