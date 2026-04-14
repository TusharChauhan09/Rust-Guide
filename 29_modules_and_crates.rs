// ============================================================
// 29 - MODULES, CRATES, AND PACKAGES
// ============================================================
// THE RUST "MODULE SYSTEM":
//
//   Package  -> one or more crates; has a Cargo.toml
//   Crate    -> smallest unit of compilation
//                 - binary crate: has fn main(), produces executable
//                 - library crate: no main, produces a .rlib
//   Module   -> organizes code within a crate (namespaces)
//   Path     -> how to refer to items (foo::bar::Baz)
//
// FILE LAYOUT CONVENTIONS:
//
//   my_package/
//     Cargo.toml
//     src/
//       main.rs           <- binary crate root (fn main)
//       lib.rs            <- library crate root
//       bin/
//         tool.rs         <- additional binary crates
//       front_of_house.rs         (module)
//       front_of_house/
//         hosting.rs              (submodule)
//
// VISIBILITY:
//   pub                    -> public to users of the module
//   pub(crate)             -> visible inside the crate only
//   pub(super)             -> visible to parent module
//   pub(in some::path)     -> scoped visibility
//   (no pub)               -> private to the module (default)
// ============================================================

// ---------- DEFINING MODULES IN ONE FILE ----------
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("added to waitlist");
        }
        fn seat_at_table() {}        // private
    }

    pub mod serving {
        pub fn take_order() {
            println!("order taken");
        }
    }
}

// ---------- PATHS: absolute vs relative ----------
pub fn eat_at_restaurant() {
    // absolute path (starts with crate or crate name)
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path (from current module)
    front_of_house::serving::take_order();
}

// ---------- use BRINGS PATHS INTO SCOPE ----------
use crate::front_of_house::hosting;

pub fn run() {
    hosting::add_to_waitlist();
}

// ---------- RE-EXPORTING with pub use ----------
pub use crate::front_of_house::serving;
// now external callers can do `my_crate::serving::take_order()`

// ---------- NESTED use AND GLOB ----------
// use std::{cmp::Ordering, io};             // two imports
// use std::io::{self, Write};               // std::io and std::io::Write
// use std::collections::*;                  // everything public

// ---------- RENAMING with as ----------
use std::io::Result as IoResult;
fn _example() -> IoResult<()> { Ok(()) }

// ---------- super AND self ----------
mod outer {
    pub fn outer_fn() { println!("outer_fn"); }

    pub mod inner {
        pub fn call_outer() {
            super::outer_fn();    // refers to parent module
        }
        pub fn call_self() {
            self::another();
        }
        fn another() { println!("another"); }
    }
}

// ---------- STRUCTS & ENUMS AND pub ----------
mod house {
    pub struct Breakfast {
        pub toast: String,          // public field
        seasonal_fruit: String,     // private field
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: toast.into(),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // All variants of a pub enum are public automatically.
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn main() {
    eat_at_restaurant();
    run();

    let mut b = house::Breakfast::summer("rye");
    b.toast = String::from("wheat");    // public field, OK
    // b.seasonal_fruit = ...          // ERROR: private

    let _a = house::Appetizer::Soup;

    outer::outer_fn();
    outer::inner::call_outer();
    outer::inner::call_self();
}

// ============================================================
// CRATES & DEPENDENCIES (Cargo.toml)
// ============================================================
// [package]
// name = "my_crate"
// version = "0.1.0"
// edition = "2021"
//
// [dependencies]
// serde = { version = "1", features = ["derive"] }
// rand = "0.8"
//
// USEFUL CARGO COMMANDS:
//   cargo new NAME          -> create a package
//   cargo new NAME --lib    -> library
//   cargo build             -> debug build
//   cargo build --release   -> optimized
//   cargo run               -> build & run
//   cargo test              -> run tests
//   cargo doc --open        -> generate & view docs
//   cargo add some_crate    -> add dependency
//   cargo update            -> bump deps per semver
//
// WORKSPACES (one Cargo.toml managing multiple crates):
//   [workspace]
//   members = ["crate_a", "crate_b"]
// ============================================================
