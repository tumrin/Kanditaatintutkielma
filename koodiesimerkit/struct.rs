// Create type person with struct
struct Person {
    age: i32,
    first_name: String,
    last_name: String,
}

// Implement methods for Person
impl Person {
    /// This creates a new Person with specified parameters
    ///
    ///  # Arguments
    ///
    /// * `age` - A integer which holds Person's age
    /// * `first_name` - A string slice of Person's first name
    /// * `last_name` - A string slice of Person's last name
    pub fn new(age: i32, first_name: &str, last_name: &str) -> Person {
        Person {
            age,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }
}

let person = Person::new(20, "John", "Doe"); // Create new Person