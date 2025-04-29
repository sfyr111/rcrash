// ch05_04_struct_visibility.rs
// Demonstration of struct and field visibility in Rust

mod mymod {
    // Struct is public, but fields are private by default
    pub struct PublicStruct {
        // Public field
        pub x: i32,
        // Private field
        y: i32,
    }

    impl PublicStruct {
        // Public constructor
        pub fn new(x: i32, y: i32) -> Self {
            PublicStruct { x, y }
        }
        // Public getter for private field y
        pub fn get_y(&self) -> i32 {
            self.y
        }
        // Public setter for private field y
        pub fn set_y(&mut self, new_y: i32) {
            self.y = new_y;
        }
        // Public setter for public field x (demonstration)
        pub fn set_x(&mut self, new_x: i32) {
            self.x = new_x;
        }
        // Public getter for x
        pub fn get_x(&self) -> i32 {
            self.x
        }
    }

    // Entire struct is private (default)
    struct PrivateStruct {
        pub x: i32, // Even if field is pub, struct is not accessible outside
    }

    // Struct and all fields are public
    pub struct AllPublic {
        pub a: i32,
        pub b: i32,
    }
}

fn main() {
    // Accessing public struct and public field
    let mut s = mymod::PublicStruct::new(1, 2);
    println!("PublicStruct.x = {}", s.x);
    // println!("PublicStruct.y = {}", s.y); // Error: field `y` is private
    println!("PublicStruct.y = {}", s.get_y());

    // Mutate fields using setters
    s.set_x(10);
    s.set_y(20);
    println!("After set: PublicStruct.x = {}, y = {}", s.get_x(), s.get_y());

    // let p = mymod::PrivateStruct { x: 10 }; // Error: struct is private

    let mut ap = mymod::AllPublic { a: 5, b: 6 };
    println!("AllPublic.a = {}, b = {}", ap.a, ap.b);
    // Directly mutate public fields
    ap.a = 50;
    ap.b = 60;
    println!("After set: AllPublic.a = {}, b = {}", ap.a, ap.b);
}
