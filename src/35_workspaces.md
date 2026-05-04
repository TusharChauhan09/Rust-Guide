# Cargo workspaces

Workspaces are equivalents of monorepos in rust

As your project develops, you might find that the library crate continues to get bigger and you want to split your package further into multiple library crates. Cargo offers a feature called *workspaces* that can help manage multiple related packages that are developed in tandem.

### Creating a workspace

Create a new folder

```rust
mkdir add_app
```

Add a Cargo.toml file

```rust
[workspace]
resolver = "2"
```

Add a new member

```rust
cargo init --lib adder
```

Add some functionality to the `adder` crate

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

Add the main module

```rust
cargo init --bin main
```

Update dependencies section in the `main` Cargo.toml

```rust
use adder::add;

fn main() {
    println!("Hello, world!");
    println!("{}", add(1, 2));
}
```

Run the main module

```rust
cargo run -p main
```