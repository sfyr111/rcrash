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
│       └── ...                         # More chapter demos
├── docs/
│   ├── ch03_3_var_mut.md               # 3.3 Documentation
│   ├── ch03_4_basic_types.md           # 3.4 Documentation
│   ├── ch03_6_integer_overflow.md      # 3.6 Documentation
│   ├── ch03_7_tuple.md                 # 3.7 Documentation
│   ├── ch03_8_array.md                 # 3.8 Documentation
│   ├── ch03_9_slice.md                 # 3.9 Documentation
│   ├── ch03_10_struct.md               # 3.10 Documentation
│   ├── ch03_11_enum.md                 # 3.11 Documentation
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

> More chapters coming soon...

---

## Notes

- Each chapter's demo file and documentation are directly linked for easy access.
- It is recommended to use this project together with the [Rust Book](https://doc.rust-lang.org/book/).
- Note that printing for enums is now manual (no Debug).
- Contributions, feedback, and additional chapters are welcome!
