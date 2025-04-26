# Rust Crash Course Demos

This project contains code demos and markdown explanations for a Rust programming course. It is suitable for self-study, teaching, and quick reference.

---

## Project Structure

```
rcrash/
├── src/
│   └── bin/
│       ├── ch03_3_var_mut.rs           # 3.3 Variables and Mutability
│       ├── ch03_4_basic_types.rs       # 3.4 Basic Data Types
│       ├── ch03_6_integer_overflow.rs  # 3.6 Integer Overflow
│       └── ...                         # More chapter demos
├── docs/
│   ├── ch03_3_var_mut.md               # 3.3 Documentation
│   ├── ch03_4_basic_types.md           # 3.4 Documentation
│   ├── ch03_6_integer_overflow.md      # 3.6 Documentation
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
```

---

## Chapter Index

| Chapter | Topic                   | Demo File                                                        | Documentation                                                      |
|---------|-------------------------|-------------------------------------------------------------------|--------------------------------------------------------------------|
| 3.3     | Variables and Mutability| [ch03_3_var_mut.rs](src/bin/ch03_3_var_mut.rs)                   | [ch03_3_var_mut.md](docs/ch03_3_var_mut.md)                        |
| 3.4     | Basic Data Types        | [ch03_4_basic_types.rs](src/bin/ch03_4_basic_types.rs)           | [ch03_4_basic_types.md](docs/ch03_4_basic_types.md)                |
| 3.6     | Integer Overflow        | [ch03_6_integer_overflow.rs](src/bin/ch03_6_integer_overflow.rs) | [ch03_6_integer_overflow.md](docs/ch03_6_integer_overflow.md)      |

> More chapters coming soon...

---

## Notes

- Each chapter's demo file and documentation are directly linked for easy access.
- It is recommended to use this project together with the [Rust Book](https://doc.rust-lang.org/book/).
- Contributions, feedback, and additional chapters are welcome!
