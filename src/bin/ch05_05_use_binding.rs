// ch05_05_use_binding.rs
// Demonstrates using `use` to bring module members into scope in Rust

mod outer {
    pub mod inner {
        pub fn greet() {
            println!("Hello from inner::greet()");
        }
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        pub const MAGIC: i32 = 42;
    }
}

// Importing members with `use`
use outer::inner::greet;
use outer::inner::add as add_fn;
use outer::inner::MAGIC;

fn main() {
    // Call imported function directly
    greet();
    let sum = add_fn(3, 4);
    println!("add_fn(3, 4) = {}", sum);
    println!("MAGIC = {}", MAGIC);

    // Access without `use`
    let sum2 = outer::inner::add(10, 20);
    println!("outer::inner::add(10, 20) = {}", sum2);
}
