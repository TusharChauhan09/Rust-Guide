# Libs vs binaries

### Libraries

A **library** is a collection of reusable code that can be used by other programs or libraries. In Rust a library is a compiled version of Rust code that doesn't have a main entry point. It contains functions, structs, and other components that other code can use.

```rust
cargo init --lib

```

### Binaries

A **binary** is a program or executable that can be run directly on an operating system. In Rust, a binary is usually a `.exe` (on Windows) or an executable file (on Unix-based systems like Linux or macOS).

A binary project in Rust is defined by a file named `main.rs` (or other files in the `src` directory). When you compile a binary, Rust generates a standalone executable file. The main purpose of the binary is to be run by the user, typically with some kind of entry point (`fn main()`).

```rust
cargo init --bin
```