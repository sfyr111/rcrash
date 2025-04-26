# Chapter 3.10: Struct

## Key Points
- Structs are custom data types that let you name and package multiple related values together.
- Three forms: regular structs, tuple structs, and unit structs.
- Use `#[derive(Debug)]` to print the entire struct with `{:?}`.
- Associated functions (like `fn new`) often act as constructors, taking no `self` parameter and called with `User::new(...)`.
- Associated methods (like `fn show(&self)`) act on an instance, called with `user.show()`.
- Use the `..` syntax to quickly create a new struct from another instance.
- **Note: If a tuple struct field is not accessed individually, there will be a dead_code warning. It is recommended to explicitly access the field like `println!("Color fields: {}, {}, {}", black.0, black.1, black.2);`.**

## Demo Code

```rust
fn main() {
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u32,
        active: bool,
    }

    impl User {
        fn new(username: &str, email: &str) -> User {
            User {
                username: username.to_string(),
                email: email.to_string(),
                sign_in_count: 1,
                active: true,
            }
        }
        fn show(&self) {
            println!("User info: username={}, email={}, sign_in_count={}, active={}",
                self.username, self.email, self.sign_in_count, self.active);
        }
    }
    let mut user1 = User::new("alice", "alice@example.com");
    println!("user1 = {:?}", user1); // print once with {:?}
    user1.show(); // print with method

    // accessing and modifying fields
    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Active: {}", user1.active);
    println!("Sign in count: {}", user1.sign_in_count);
    user1.email = String::from("alice@newmail.com");
    println!("Updated email: {}", user1.email);
    user1.show();

    // struct update syntax
    let user2 = User {
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        ..user1
    };
    println!("user2 = {:?}", user2);
    user2.show();

    // tuple struct
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("black = {:?}", black);
    // use fields to avoid dead_code warning
    println!("Color fields: {}, {}, {}", black.0, black.1, black.2);

    // unit struct
    #[derive(Debug)]
    struct Marker;
    let m = Marker;
    println!("marker = {:?}", m);
}
```

## Table: Struct Features

| Feature                 | Example                                      | Notes                                   |
|-------------------------|----------------------------------------------|-----------------------------------------|
| Define struct           | `struct User { ... }`                        | Named fields                            |
| Create instance         | `let u = User::new(...)`                     | 推荐用 new 构造器                       |
| Associated function     | `impl User { fn new(...) -> User { ... } }`  | 构造器/辅助函数 via `impl`              |
| Associated method       | `impl User { fn show(&self) { ... } }`       | 用于实例，`u.show()`                     |
| Print whole struct      | `println!("{:?}", u)`                        | 需 `#[derive(Debug)]`                    |
| Access field            | `u.username`                                 | Use dot syntax                          |
| Mutable instance        | `let mut u = User { ... }`                   | Allows modifying fields                 |
| Update field            | `u.email = ...`                              | Only if instance is mutable             |
| Struct update syntax    | `User { ..u1 }`                              | Copy remaining fields from another      |
| Tuple struct            | `struct Color(i32, i32, i32);`               | Fields have no names                    |
| Tuple struct access     | `c.0, c.1, c.2`                              | Access by index，避免 dead_code 警告      |
| Unit-like struct        | `struct Marker;`                             | No fields, just a marker                |
| Unit-like instance      | `let m = Marker;`                            | Used for marker types                   |

## Notes
- Use `#[derive(Debug)]` + `{:?}` for easy debugging and one-time printing of structs.
- Associated functions are often used as constructors, while associated methods act on instances.
- Tuple struct fields will have a dead_code warning if they are not accessed individually, so it is recommended to access them explicitly.
- Named fields make code more readable and reduce errors.
- Tuple structs are suitable for when fields have no meaning, and unit structs are often used for type markers or traits.

---

> For more details, see [Rust Book: Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
