// ch05_06_super_self.rs
// Demonstrates using `super` and `self` to simplify module paths in Rust

mod outer {
    pub fn outer_fn() {
        println!("Called outer_fn()");
    }
    pub mod inner {
        pub fn inner_fn() {
            println!("Called inner_fn()");
        }
        pub fn call_outer_direct() {
            // Use super to access parent module
            super::outer_fn();
        }
        pub fn call_inner_self() {
            // Use self to call a function in the same module
            self::inner_fn();
        }
        pub fn call_both() {
            self::call_inner_self();
            super::outer_fn();
        }
    }
}

fn main() {
    // Call from main
    outer::outer_fn();
    outer::inner::inner_fn();
    outer::inner::call_outer_direct();
    outer::inner::call_inner_self();
    outer::inner::call_both();
}
