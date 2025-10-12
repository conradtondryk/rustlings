// enums16.rs
//
// Advanced exercise: Build a JSON-like data structure using enums.
// You'll need to define recursive enums and implement complex methods.
//
// Execute `rustlings hint enums16` or use the `hint` watch subcommand for a
// hint.

// TODO: Define an enum called `JsonValue` with these variants:
// - Null
// - Bool(bool)
// - Number(f64)
// - String(String)
// - Array(Vec<JsonValue>)
// - Object(Vec<(String, JsonValue)>)  // Vec of key-value pairs
//
// Note: This enum is recursive - Array and Object contain JsonValue!

// TODO: Implement these methods on JsonValue:
//
// - new_null() -> Self
//   Creates a Null value
//
// - new_bool(value: bool) -> Self
//   Creates a Bool value
//
// - new_number(value: f64) -> Self
//   Creates a Number value
//
// - new_string(value: String) -> Self
//   Creates a String value
//
// - new_array() -> Self
//   Creates an empty Array
//
// - new_object() -> Self
//   Creates an empty Object
//
// - is_null(&self) -> bool
//   Returns true if this is a Null value
//
// - as_bool(&self) -> Option<bool>
//   Returns Some(bool) if this is a Bool, None otherwise
//
// - as_number(&self) -> Option<f64>
//   Returns Some(f64) if this is a Number, None otherwise
//
// - push(&mut self, value: JsonValue)
//   If self is an Array, pushes the value to it. Otherwise, does nothing.
//
// - insert(&mut self, key: String, value: JsonValue)
//   If self is an Object, inserts the key-value pair. Otherwise, does nothing.

fn main() {
    // Create a JSON object like: {"name": "Alice", "age": 30, "active": true}
    let mut person = JsonValue::new_object();
    person.insert(String::from("name"), JsonValue::new_string(String::from("Alice")));
    person.insert(String::from("age"), JsonValue::new_number(30.0));
    person.insert(String::from("active"), JsonValue::new_bool(true));

    // Create a JSON array like: [1, 2, 3]
    let mut numbers = JsonValue::new_array();
    numbers.push(JsonValue::new_number(1.0));
    numbers.push(JsonValue::new_number(2.0));
    numbers.push(JsonValue::new_number(3.0));

    let null_value = JsonValue::new_null();
    println!("Is null? {}", null_value.is_null());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null() {
        let val = JsonValue::new_null();
        assert!(val.is_null());
    }

    #[test]
    fn test_bool() {
        let val = JsonValue::new_bool(true);
        assert_eq!(val.as_bool(), Some(true));
        assert_eq!(val.as_number(), None);
    }

    #[test]
    fn test_number() {
        let val = JsonValue::new_number(42.5);
        assert_eq!(val.as_number(), Some(42.5));
        assert_eq!(val.as_bool(), None);
    }

    #[test]
    fn test_array_push() {
        let mut arr = JsonValue::new_array();
        arr.push(JsonValue::new_number(1.0));
        arr.push(JsonValue::new_number(2.0));

        if let JsonValue::Array(vec) = arr {
            assert_eq!(vec.len(), 2);
        } else {
            panic!("Expected Array variant");
        }
    }

    #[test]
    fn test_object_insert() {
        let mut obj = JsonValue::new_object();
        obj.insert(String::from("key1"), JsonValue::new_string(String::from("value1")));
        obj.insert(String::from("key2"), JsonValue::new_number(123.0));

        if let JsonValue::Object(pairs) = obj {
            assert_eq!(pairs.len(), 2);
        } else {
            panic!("Expected Object variant");
        }
    }
}
