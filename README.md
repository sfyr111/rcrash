# Rust Crash Course Demos

This project contains code demos and markdown explanations for a Rust programming course. It is suitable for self-study, teaching, and quick reference.

---

## Project Structure

```markdown
rcrash/
├── src/
│   └── bin/
│       ├── ch03_3_var_mut.rs           # 3.3 Variables and Mutability
│       ├── ch03_4_basic_types.rs       # 3.4 Basic Data Types
│       ├── ch03_6_integer_overflow.rs  # 3.6 Integer Overflow
│       ├── ch03_7_tuple.rs             # 3.7 Tuple
│       ├── ch03_8_array.rs             # 3.8 Array
│       ├── ch03_9_slice.rs             # 3.9 Slice Type
│       ├── ch03_10_struct.rs           # 3.10 Struct
│       ├── ch03_11_enum.rs             # 3.11 Enum
│       ├── ch03_12_comment.rs          # 3.12 Comment Types
│       ├── ch03_13_println.rs          # 3.13 println! Macro
│       ├── ch03_14_cast.rs             # 3.14 Type Conversion
│       ├── ch04_2_expr.rs              # 4.2 Expression Forms
│       ├── ch04_3_if.rs                # 4.3 if/else Control Flow
│       ├── ch04_4_loop.rs              # 4.4 Using loop
│       ├── ch04_5_while.rs             # 4.5 Using while
│       ├── ch04_6_for.rs               # 4.6 Using for/range
│       ├── ch04_7_match.rs             # 4.7 match Syntax
│       ├── ch04_8_if_let.rs            # 4.8 if let Syntactic Sugar
│       ├── ch04_9_while_let.rs         # 4.9 while let Syntactic Sugar
│       ├── ch04_10_fn_method.rs        # 4.10 Functions and Methods
│       ├── ch04_11_fn_closure.rs       # 4.11 Functions and Closures
│       ├── ch04_13_high_order.rs       # 4.13 High-Order Functions
│       ├── ch04_14_never.rs            # 4.14 Diverging Functions (Never Type)
│       ├── ch04_15_guess.rs            # 4.15 Guessing Game
│       ├── ch05_02_mod.rs               # 5.2 Modular Programming
│       ├── ch05_03_pub.rs               # 5.3 Visibility with pub
│       ├── ch05_04_struct_visibility.rs # 5.4 Struct Visibility
│       ├── ch05_05_use_binding.rs       # 5.5 Using `use` Binding
│       ├── ch05_06_super_self.rs        # 5.6 Using `super` and `self`
│       ├── ch05_08_project_structure.md # 5.8 Project Directory Structure
│       ├── ch06_02_generic_fn_param.rs  # 6.2 Generics as Function Parameter Types
│       ├── ch06_03_generic_struct.rs    # 6.3 Generics in Structs
│       ├── ch06_04_generic_struct_impl.rs # 6.4 Implementations for Generic Structs
│       ├── ch06_05_traits.rs            # 6.5 Defining Shared Behavior with Traits
│       ├── ch06_06_derive.rs            # 6.6 Automatic Derivation
│       ├── ch07_02_ownership.rs         # 7.2 Ownership in Rust
│       ├── ch07_03_borrowing.rs         # 7.3 Borrowing in Rust
│       ├── ch07_04_lifetime.rs          # 7.4 Lifetimes in Rust
│       ├── ch07_05_lifetime_annotation.rs # 7.5 Lifetime Annotations in Rust
│       ├── ch08_02_unrecoverable.rs     # 8.2 Unrecoverable Errors in Rust
│       └── ...                         # More chapter demos
├── docs/
│   ├── ch03_3_var_mut.md               # 3.3 Variables and Mutability
│   ├── ch03_4_basic_types.md           # 3.4 Basic Data Types
│   ├── ch03_6_integer_overflow.md      # 3.6 Integer Overflow
│   ├── ch03_7_tuple.md                 # 3.7 Tuple
│   ├── ch03_8_array.md                 # 3.8 Array
│   ├── ch03_9_slice.md                 # 3.9 Slice Type
│   ├── ch03_10_struct.md               # 3.10 Struct
│   ├── ch03_11_enum.md                 # 3.11 Enum
│   ├── ch03_12_comment.md              # 3.12 Comment Types
│   ├── ch03_13_println.md              # 3.13 println! Macro
│   ├── ch03_14_cast.md                 # 3.14 Type Conversion
│   ├── ch04_2_expr.md                  # 4.2 Expression Forms
│   ├── ch04_3_if.md                    # 4.3 if/else Control Flow
│   ├── ch04_4_loop.md                  # 4.4 Using loop
│   ├── ch04_5_while.md                 # 4.5 Using while
│   ├── ch04_6_for.md                   # 4.6 Using for/range
│   ├── ch04_7_match.md                 # 4.7 match Syntax
│   ├── ch04_8_if_let.md                # 4.8 if let Syntactic Sugar
│   ├── ch04_9_while_let.md             # 4.9 while let Syntactic Sugar
│   ├── ch04_10_fn_method.md            # 4.10 Functions and Methods
│   ├── ch04_11_fn_closure.md           # 4.11 Functions and Closures
│   ├── ch04_13_high_order.md           # 4.13 High-Order Functions
│   ├── ch04_14_never.md                # 4.14 Diverging Functions (Never Type)
│   ├── ch04_15_guess.md                # 4.15 Guessing Game
│   ├── ch05_02_mod.md                   # 5.2 Modular Programming
│   ├── ch05_03_pub.md                   # 5.3 Visibility with pub
│   ├── ch05_04_struct_visibility.md     # 5.4 Struct Visibility
│   ├── ch05_05_use_binding.md           # 5.5 Using `use` Binding
│   ├── ch05_06_super_self.md            # 5.6 Using `super` and `self`
│   ├── ch05_08_project_structure.md     # 5.8 Project Directory Structure
│   ├── ch06_02_generic_fn_param.md      # 6.2 Generics as Function Parameter Types
│   ├── ch06_03_generic_struct.md        # 6.3 Generics in Structs
│   ├── ch06_04_generic_struct_impl.md   # 6.4 Implementations for Generic Structs
│   ├── ch06_05_traits.md                # 6.5 Defining Shared Behavior with Traits
│   ├── ch06_06_derive.md                # 6.6 Automatic Derivation
│   ├── ch07_02_ownership.md             # 7.2 Ownership in Rust
│   ├── ch07_03_borrowing.md             # 7.3 Borrowing in Rust
│   ├── ch07_04_lifetime.md              # 7.4 Lifetimes in Rust
│   ├── ch07_05_lifetime_annotation.md   # 7.5 Lifetime Annotations in Rust
│   ├── ch08_02_unrecoverable.md         # 8.2 Unrecoverable Errors in Rust
│   └── ...                             # More chapter docs
└── README.md                           # Project introduction and index
```

---

## Quick Start

1. Install the Rust toolchain: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Clone or download this repository
3. Run any chapter demo, for example:

```bash
cargo run --bin ch03_3_var_mut
cargo run --bin ch03_4_basic_types
cargo run --bin ch03_6_integer_overflow
cargo run --bin ch03_7_tuple
cargo run --bin ch03_8_array
cargo run --bin ch03_9_slice
cargo run --bin ch03_10_struct
cargo run --bin ch03_11_enum
cargo run --bin ch03_12_comment
cargo run --bin ch03_13_println
cargo run --bin ch03_14_cast
cargo run --bin ch04_2_expr
cargo run --bin ch04_3_if
cargo run --bin ch04_4_loop
cargo run --bin ch04_5_while
cargo run --bin ch04_6_for
cargo run --bin ch04_7_match
cargo run --bin ch04_8_if_let
cargo run --bin ch04_9_while_let
cargo run --bin ch04_10_fn_method
cargo run --bin ch04_11_fn_closure
cargo run --bin ch04_13_high_order
cargo run --bin ch04_14_never
cargo run --bin ch04_15_guess
cargo run --bin ch05_02_mod
cargo run --bin ch05_03_pub
cargo run --bin ch05_04_struct_visibility
cargo run --bin ch05_05_use_binding
cargo run --bin ch05_06_super_self
cargo run --bin ch06_02_generic_fn_param
cargo run --bin ch06_03_generic_struct
cargo run --bin ch06_04_generic_struct_impl
cargo run --bin ch06_05_traits
cargo run --bin ch06_06_derive
cargo run --bin ch07_02_ownership
cargo run --bin ch07_03_borrowing
cargo run --bin ch07_04_lifetime
cargo run --bin ch07_05_lifetime_annotation
cargo run --bin ch08_02_unrecoverable
```

---

## Chapter Index

---

| #   | Chapter & Topic                  | Demo File                                                        | Documentation                                                      |
|-----|-----------------------------------|-------------------------------------------------------------------|--------------------------------------------------------------------|
| 3.3 | Variables and Mutability          | [`ch03_3_var_mut.rs`](src/bin/ch03_3_var_mut.rs)                   | [`ch03_3_var_mut.md`](docs/ch03_3_var_mut.md)                        |
| 3.4 | Basic Data Types                  | [`ch03_4_basic_types.rs`](src/bin/ch03_4_basic_types.rs)           | [`ch03_4_basic_types.md`](docs/ch03_4_basic_types.md)                |
| 3.6 | Integer Overflow                  | [`ch03_6_integer_overflow.rs`](src/bin/ch03_6_integer_overflow.rs) | [`ch03_6_integer_overflow.md`](docs/ch03_6_integer_overflow.md)      |
| 3.7 | Tuple                             | [`ch03_7_tuple.rs`](src/bin/ch03_7_tuple.rs)                       | [`ch03_7_tuple.md`](docs/ch03_7_tuple.md)                            |
| 3.8 | Array                             | [`ch03_8_array.rs`](src/bin/ch03_8_array.rs)                       | [`ch03_8_array.md`](docs/ch03_8_array.md)                            |
| 3.9 | Slice Type                        | [`ch03_9_slice.rs`](src/bin/ch03_9_slice.rs)                       | [`ch03_9_slice.md`](docs/ch03_9_slice.md)                            |
| 3.10| Struct                            | [`ch03_10_struct.rs`](src/bin/ch03_10_struct.rs)                   | [`ch03_10_struct.md`](docs/ch03_10_struct.md)                        |
| 3.11| Enum                              | [`ch03_11_enum.rs`](src/bin/ch03_11_enum.rs)                       | [`ch03_11_enum.md`](docs/ch03_11_enum.md)                            |
| 3.12| Comment Types                     | [`ch03_12_comment.rs`](src/bin/ch03_12_comment.rs)                 | [`ch03_12_comment.md`](docs/ch03_12_comment.md)                      |
| 3.13| println! Macro                    | [`ch03_13_println.rs`](src/bin/ch03_13_println.rs)                 | [`ch03_13_println.md`](docs/ch03_13_println.md)                      |
| 3.14| Type Conversion                   | [`ch03_14_cast.rs`](src/bin/ch03_14_cast.rs)                       | [`ch03_14_cast.md`](docs/ch03_14_cast.md)                            |
| 4.2 | Expression Forms                  | [`ch04_2_expr.rs`](src/bin/ch04_2_expr.rs)                         | [`ch04_2_expr.md`](docs/ch04_2_expr.md)                              |
| 4.3 | if/else Control Flow              | [`ch04_3_if.rs`](src/bin/ch04_3_if.rs)                             | [`ch04_3_if.md`](docs/ch04_3_if.md)                                  |
| 4.4 | Using loop                        | [`ch04_4_loop.rs`](src/bin/ch04_4_loop.rs)                         | [`ch04_4_loop.md`](docs/ch04_4_loop.md)                              |
| 4.5 | Using while                       | [`ch04_5_while.rs`](src/bin/ch04_5_while.rs)                       | [`ch04_5_while.md`](docs/ch04_5_while.md)                            |
| 4.6 | Using for/range                   | [`ch04_6_for.rs`](src/bin/ch04_6_for.rs)                           | [`ch04_6_for.md`](docs/ch04_6_for.md)                                |
| 4.7 | match Syntax                      | [`ch04_7_match.rs`](src/bin/ch04_7_match.rs)                       | [`ch04_7_match.md`](docs/ch04_7_match.md)                            |
| 4.8 | if let Syntactic Sugar            | [`ch04_8_if_let.rs`](src/bin/ch04_8_if_let.rs)                     | [`ch04_8_if_let.md`](docs/ch04_8_if_let.md)                          |
| 4.9 | while let Syntactic Sugar         | [`ch04_9_while_let.rs`](src/bin/ch04_9_while_let.rs)               | [`ch04_9_while_let.md`](docs/ch04_9_while_let.md)                    |
| 4.10| Functions and Methods             | [`ch04_10_fn_method.rs`](src/bin/ch04_10_fn_method.rs)              | [`ch04_10_fn_method.md`](docs/ch04_10_fn_method.md)                   |
| 4.11| Functions and Closures            | [`ch04_11_fn_closure.rs`](src/bin/ch04_11_fn_closure.rs)            | [`ch04_11_fn_closure.md`](docs/ch04_11_fn_closure.md)                 |
| 4.13| High-Order Functions              | [`ch04_13_high_order.rs`](src/bin/ch04_13_high_order.rs)            | [`ch04_13_high_order.md`](docs/ch04_13_high_order.md)                 |
| 4.14| Diverging Functions (Never)       | [`ch04_14_never.rs`](src/bin/ch04_14_never.rs)                      | [`ch04_14_never.md`](docs/ch04_14_never.md)                           |
| 4.15| Guessing Game                     | [`ch04_15_guess.rs`](src/bin/ch04_15_guess.rs)                      | [`ch04_15_guess.md`](docs/ch04_15_guess.md)                           |
| 5.2 | Modular Programming               | [`ch05_02_mod.rs`](src/bin/ch05_02_mod.rs)                          | [`ch05_02_mod.md`](docs/ch05_02_mod.md)                              |
| 5.3 | Visibility with pub               | [`ch05_03_pub.rs`](src/bin/ch05_03_pub.rs)                          | [`ch05_03_pub.md`](docs/ch05_03_pub.md)                              |
| 5.4 | Struct Visibility                 | [`ch05_04_struct_visibility.rs`](src/bin/ch05_04_struct_visibility.rs) | [`ch05_04_struct_visibility.md`](docs/ch05_04_struct_visibility.md) |
| 5.5 | Using `use` Binding               | [`ch05_05_use_binding.rs`](src/bin/ch05_05_use_binding.rs)             | [`ch05_05_use_binding.md`](docs/ch05_05_use_binding.md)             |
| 5.6 | Using `super` and `self`          | [`ch05_06_super_self.rs`](src/bin/ch05_06_super_self.rs)               | [`ch05_06_super_self.md`](docs/ch05_06_super_self.md)               |
| 5.8 | Project Directory Structure        | —                                                               | [`ch05_08_project_structure.md`](docs/ch05_08_project_structure.md) |
| 6.2 | Generics as Function Parameter Types | [`ch06_02_generic_fn_param.rs`](src/bin/ch06_02_generic_fn_param.rs) | [`ch06_02_generic_fn_param.md`](docs/ch06_02_generic_fn_param.md) |
| 6.3 | Generics in Structs                 | [`ch06_03_generic_struct.rs`](src/bin/ch06_03_generic_struct.rs) | [`ch06_03_generic_struct.md`](docs/ch06_03_generic_struct.md) |
| 6.4 | Implementations for Generic Structs | [`ch06_04_generic_struct_impl.rs`](src/bin/ch06_04_generic_struct_impl.rs) | [`ch06_04_generic_struct_impl.md`](docs/ch06_04_generic_struct_impl.md) |
| 6.5 | Defining Shared Behavior with Traits | [`ch06_05_traits.rs`](src/bin/ch06_05_traits.rs) | [`ch06_05_traits.md`](docs/ch06_05_traits.md) |
| 6.6 | Automatic Derivation | [`ch06_06_derive.rs`](src/bin/ch06_06_derive.rs) | [`ch06_06_derive.md`](docs/ch06_06_derive.md) |
| 7.2 | Ownership in Rust | [`ch07_02_ownership.rs`](src/bin/ch07_02_ownership.rs) | [`ch07_02_ownership.md`](docs/ch07_02_ownership.md) |
| 7.3 | Borrowing in Rust | [`ch07_03_borrowing.rs`](src/bin/ch07_03_borrowing.rs) | [`ch07_03_borrowing.md`](docs/ch07_03_borrowing.md) |
| 7.4 | Lifetimes in Rust | [`ch07_04_lifetime.rs`](src/bin/ch07_04_lifetime.rs) | [`ch07_04_lifetime.md`](docs/ch07_04_lifetime.md) |
| 7.5 | Lifetime Annotations in Rust | [`ch07_05_lifetime_annotation.rs`](src/bin/ch07_05_lifetime_annotation.rs) | [`ch07_05_lifetime_annotation.md`](docs/ch07_05_lifetime_annotation.md) |
| 8.2 | Unrecoverable Errors in Rust | [`ch08_02_unrecoverable.rs`](src/bin/ch08_02_unrecoverable.rs) | [`ch08_02_unrecoverable.md`](docs/ch08_02_unrecoverable.md) |
| ... | ...                               | ...                                                               | ...                                                               |

> More chapters coming soon...

---

## Notes

- Each chapter's demo file and documentation are directly linked for easy access.
- It is recommended to use this project together with the [Rust Book](https://doc.rust-lang.org/book/).
- Note that printing for enums is now manual (no Debug).
- Contributions, feedback, and additional chapters are welcome!
