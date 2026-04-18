// ============================================================
// 20 - TRAITS
// ============================================================
// Traits define shared behavior that types can implement.
// They are similar to interfaces in other languages but more
// powerful (default methods, generic bounds, associated
// types, etc.).
//
// SYNTAX:
//   trait Name {
//       fn method(&self) -> T;       // required
//       fn other(&self) -> T { ... } // default implementation
//   }
//
//   impl Name for MyType { ... }
//
// THE "ORPHAN RULE":
//   You can implement a trait on a type only if either the
//   trait OR the type is local to your crate.
// ============================================================

use std::fmt::Display;

// ---------- DEFINING A TRAIT ----------
trait Summary {
    // required method - must be implemented by types that implement this trait
    fn summarize(&self) -> String;

    // default method - types can override or if not deuse as-is
    fn short(&self) -> String {
        format!("({}...)", self.summarize())
    }
}

// ---------- IMPLEMENTING A TRAIT ----------
struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("'{}' by {}", self.title, self.author)
    }
}

struct Tweet {
    user: String,
    text: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.user, self.text)
    }
    // override default
    fn short(&self) -> String {
        format!("@{}", self.user)
    }
}

// ---------- TRAITS AS FUNCTION PARAMS ----------
// impl Trait syntax (sugar)
fn notify(item: &impl Summary) {
    println!("Breaking: {}", item.summarize());
}

// trait bound syntax (equivalent)
fn notify2<T: Summary>(item: &T) {
    println!("Breaking: {}", item.summarize());
}

// Multiple bounds
fn notify3<T: Summary + Display>(item: &T) {
    println!("{} -> {}", item, item.summarize());
}

// where clause for clarity
fn compare<T, U>(a: &T, b: &U) -> bool
where
    T: Summary,
    U: Summary,
{
    a.summarize() == b.summarize()
}

// ---------- RETURNING impl Trait ----------
fn make_tweet() -> impl Summary {
    Tweet {
        user: String::from("rustlang"),
        text: String::from("hello"),
    }
}

// ---------- BLANKET IMPLEMENTATIONS ----------
// Implement a trait for all types that match some bound.
// (The std library does this, e.g., impl<T: Display> ToString for T)
trait Loud {
    fn loud(&self) -> String;
}

impl<T: Display> Loud for T {
    fn loud(&self) -> String {
        format!("{}!!!", self)
    }
}

// ---------- DERIVABLE TRAITS ----------
#[derive(Debug, Clone, PartialEq)]
struct Book {
    title: String,
    pages: u32,
}

fn main() {
    let a = Article {
        title: String::from("Rust Rocks"),
        author: String::from("Jane"),
    };
    let t = Tweet {
        user: String::from("ferris"),
        text: String::from("Hi!"),
    };
    println!("{}", a.summarize());
    println!("{}", t.summarize());
    println!("{}", a.short());   // default
    println!("{}", t.short());   // overridden

    notify(&a);
    notify2(&t);

    let equal = compare(&a, &t);
    println!("equal? {}", equal);

    let newest = make_tweet();
    println!("{}", newest.summarize());

    // Blanket impl applies to anything Display
    println!("{}", 5.loud());
    println!("{}", "hi".loud());

    // Derived traits
    let b1 = Book { title: String::from("R"), pages: 200 };
    let b2 = b1.clone();
    println!("{:?}  equal? {}", b1, b1 == b2);
}
