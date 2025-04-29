# 5.4 Visibility of Structs and Fields in Rust

## Key Points

- In Rust, both structs and their fields have independent visibility.
- By default, structs and their fields are private to the module where they are defined.
- Use `pub` to make a struct or field public.
- Making a struct public does **not** make its fields public; each field needs its own visibility modifier.
- Private fields can only be accessed within the module, but you can provide public methods for controlled access.

## Example Code

```rust
mod mymod {
    // Struct is public, but fields are private by default
    pub struct PublicStruct {
        pub x: i32,      // Public field
        y: i32,         // Private field
    }

    impl PublicStruct {
        pub fn new(x: i32, y: i32) -> Self {
            PublicStruct { x, y }
        }
        pub fn get_y(&self) -> i32 {
            self.y
        }
        pub fn set_y(&mut self, new_y: i32) {
            self.y = new_y;
        }
        pub fn set_x(&mut self, new_x: i32) {
            self.x = new_x;
        }
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
```

## Table: Struct and Field Visibility

| Declaration                      | Accessible Outside Module? | Notes                                       |
|-----------------------------------|:-------------------------:|---------------------------------------------|
| `struct Foo { x: i32 }`           | ❌                        | Struct and fields are private                |
| `pub struct Foo { x: i32 }`       | ❌                        | Struct public, field private                 |
| `pub struct Foo { pub x: i32 }`   | ✅                        | Both struct and field public                 |
| `struct Foo { pub x: i32 }`       | ❌                        | Struct private, field public (has no effect) |

## Notes

- Use public methods to expose private fields if you want encapsulation.
- Making all fields public is convenient for simple data structures, but consider hiding implementation details for complex types.

---

> See also: [Rust Book - Structs and Visibility](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
