// ch06_05_traits.rs
// 6.5 Defining Shared Behavior with Traits
// Demonstrates how to use traits to define common behavior in Rust.

/// A trait for types that can describe themselves.
pub trait Describable {
    fn describe(&self) -> String;
}

/// A struct representing a Person.
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("Person: {} ({} years old)", self.name, self.age)
    }
}

/// A struct representing a Company.
pub struct Company {
    pub name: String,
    pub employee_count: usize,
}

impl Describable for Company {
    fn describe(&self) -> String {
        format!("Company: {} ({} employees)", self.name, self.employee_count)
    }
}

/// A generic function that prints the description of anything Describable.
pub fn print_description<T: Describable>(item: &T) {
    println!("{}", item.describe());
}

fn main() {
    let alice = Person { name: "Alice".to_string(), age: 30 };
    let acme = Company { name: "Acme Corp".to_string(), employee_count: 100 };

    print_description(&alice);
    print_description(&acme);
}
