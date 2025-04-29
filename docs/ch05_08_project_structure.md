# 5.8 Rust Project Directory Structure

This chapter introduces the typical directory structure of a Rust project, explaining the purpose of each main folder and file, and best practices for organizing code in real-world Rust development.

## Key Points

- Rust uses a convention-over-configuration approach for project layout.
- The `src` directory holds all source code. The entry point is usually `src/main.rs` for binaries or `src/lib.rs` for libraries.
- The `Cargo.toml` file defines project metadata, dependencies, and build configuration.
- The `Cargo.lock` file records exact dependency versions (for reproducibility).
- Additional folders like `tests`, `benches`, and `examples` are used for integration tests, benchmarks, and example code.
- Submodules can be organized as files or folders under `src`.

## Typical Rust Project Layout

```text
my_project/
├── Cargo.toml         # Project manifest (metadata & dependencies)
├── Cargo.lock         # Exact dependency versions (auto-generated)
├── src/
│   ├── main.rs        # Binary entry point (if binary project)
│   ├── lib.rs         # Library root (if library project)
│   ├── foo.rs         # Module file (mod foo;)
│   └── bar/
│       └── mod.rs     # Submodule as a folder (mod bar;)
├── tests/             # Integration tests (auto-discovered)
├── benches/           # Benchmarks (criterion or test harness)
├── examples/          # Example binaries
└── target/            # Build output (auto-generated, can ignore)
```

## More Examples: Module Hierarchies and Organization

### 1. Flat Modules (Single File per Module)

```text
src/
├── lib.rs             # mod foo; mod bar;
├── foo.rs             # pub fn foo_fn() { ... }
├── bar.rs             # pub fn bar_fn() { ... }
```

lib.rs:
```rust
mod foo;
mod bar;

pub use foo::foo_fn;
pub use bar::bar_fn;
```

### 2. Nested Modules as Folders

```text
src/
├── lib.rs             # mod utils;
├── utils/
│   ├── mod.rs         # mod string_utils; mod math_utils;
│   ├── string_utils.rs
│   └── math_utils.rs
```

lib.rs:
```rust
mod utils;
```

utils/mod.rs:
```rust
pub mod string_utils;
pub mod math_utils;
```

### 3. Deeply Nested Modules

```text
src/
├── lib.rs             # mod services;
├── services/
│   ├── mod.rs         # mod user; mod payment;
│   ├── user/
│   │   ├── mod.rs     # mod handler;
│   │   └── handler.rs
│   └── payment/
│       ├── mod.rs     # mod processor;
│       └── processor.rs
```

lib.rs:
```rust
mod services;
```

services/mod.rs:
```rust
pub mod user;
pub mod payment;
```

services/user/mod.rs:
```rust
pub mod handler;
```

services/payment/mod.rs:
```rust
pub mod processor;
```

### 4. Mixing Files and Folders

- `mod foo;` can refer to either `foo.rs` or `foo/mod.rs`.
- For larger modules, prefer the folder style for better organization.

### 5. Example: Using Modules in Code

```rust
// src/lib.rs
mod utils;

fn main() {
    utils::string_utils::do_something();
}

// src/utils/string_utils.rs
pub fn do_something() {
    println!("Did something!");
}
```

## Table: Common Folders and Files

| Name           | Purpose                                      |
|----------------|----------------------------------------------|
| Cargo.toml     | Project manifest, dependencies, metadata     |
| Cargo.lock     | Locked dependency versions                   |
| src/           | Main source code                             |
| src/main.rs    | Binary entry point                           |
| src/lib.rs     | Library crate root                           |
| tests/         | Integration tests                            |
| benches/       | Benchmarks                                   |
| examples/      | Example programs                             |
| target/        | Build artifacts, can be ignored in VCS       |

## Notes

- Place most code in `src/lib.rs` for reusability; keep `main.rs` minimal.
- Use folders for complex modules (`mod foo;` as `foo/mod.rs` or `foo.rs`).
- Integration tests go in the `tests/` directory and have full crate access.
- The `target/` directory is large and should be gitignored.
