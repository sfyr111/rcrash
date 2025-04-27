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
```

---

## Chapter Index

| Chapter | Topic                   | Demo File                                                        | Documentation                                                      |
|---------|-------------------------|-------------------------------------------------------------------|--------------------------------------------------------------------|
| 3.3     | Variables and Mutability| [ch03_3_var_mut.rs](src/bin/ch03_3_var_mut.rs)                   | [ch03_3_var_mut.md](docs/ch03_3_var_mut.md)                        |
| 3.4     | Basic Data Types        | [ch03_4_basic_types.rs](src/bin/ch03_4_basic_types.rs)           | [ch03_4_basic_types.md](docs/ch03_4_basic_types.md)                |
| 3.6     | Integer Overflow        | [ch03_6_integer_overflow.rs](src/bin/ch03_6_integer_overflow.rs) | [ch03_6_integer_overflow.md](docs/ch03_6_integer_overflow.md)      |
| 3.7     | Tuple                   | [ch03_7_tuple.rs](src/bin/ch03_7_tuple.rs)                       | [ch03_7_tuple.md](docs/ch03_7_tuple.md)                            |
| 3.8     | Array                   | [ch03_8_array.rs](src/bin/ch03_8_array.rs)                       | [ch03_8_array.md](docs/ch03_8_array.md)                            |
| 3.9     | Slice Type              | [ch03_9_slice.rs](src/bin/ch03_9_slice.rs)                       | [ch03_9_slice.md](docs/ch03_9_slice.md)                            |
| 3.10    | Struct                  | [ch03_10_struct.rs](src/bin/ch03_10_struct.rs)                   | [ch03_10_struct.md](docs/ch03_10_struct.md)                        |
| 3.11    | Enum                    | [ch03_11_enum.rs](src/bin/ch03_11_enum.rs)                       | [ch03_11_enum.md](docs/ch03_11_enum.md)                            |
| 3.12    | Comment Types           | [ch03_12_comment.rs](src/bin/ch03_12_comment.rs)                 | [ch03_12_comment.md](docs/ch03_12_comment.md)                      |
| 3.13    | println! Macro          | [ch03_13_println.rs](src/bin/ch03_13_println.rs)                 | [ch03_13_println.md](docs/ch03_13_println.md)                      |
| 3.14    | Type Conversion         | [ch03_14_cast.rs](src/bin/ch03_14_cast.rs)                       | [ch03_14_cast.md](docs/ch03_14_cast.md)                            |
| 4.2     | Expression Forms        | [ch04_2_expr.rs](src/bin/ch04_2_expr.rs)                         | [ch04_2_expr.md](docs/ch04_2_expr.md)                              |
| 4.3     | if/else Control Flow    | [ch04_3_if.rs](src/bin/ch04_3_if.rs)                             | [ch04_3_if.md](docs/ch04_3_if.md)                                  |
| 4.4     | Using loop              | [ch04_4_loop.rs](src/bin/ch04_4_loop.rs)                         | [ch04_4_loop.md](docs/ch04_4_loop.md)                              |
| 4.5     | Using while             | [ch04_5_while.rs](src/bin/ch04_5_while.rs)                       | [ch04_5_while.md](docs/ch04_5_while.md)                            |

> More chapters coming soon...

---

## Notes

- Each chapter's demo file and documentation are directly linked for easy access.
- It is recommended to use this project together with the [Rust Book](https://doc.rust-lang.org/book/).
- Note that printing for enums is now manual (no Debug).
- Contributions, feedback, and additional chapters are welcome!
