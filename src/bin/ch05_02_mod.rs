// ch05_02_mod.rs
// Demonstration of modular programming in Rust
// Showcases module declaration, visibility, and usage.

mod math {
    // Private function (default)
    fn private_add(a: i32, b: i32) -> i32 {
        a + b
    }

    // Public function
    pub fn add(a: i32, b: i32) -> i32 {
        private_add(a, b)
    }

    // Nested module
    pub mod advanced {
        pub fn square(x: i32) -> i32 {
            x * x
        }
    }
}

fn main() {
    // Using public function from module
    let sum = math::add(2, 3);
    println!("2 + 3 = {}", sum);

    // Using public function from nested module
    let squared = math::advanced::square(4);
    println!("4 squared = {}", squared);

    // The following line would not compile (private function):
    // let _ = math::private_add(2, 3);
}
