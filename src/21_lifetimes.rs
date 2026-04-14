// ============================================================
// 21 - LIFETIMES
// ============================================================
// Every reference in Rust has a lifetime: the scope for which
// the reference is valid. Usually inferred by the compiler.
// You only write lifetimes when the compiler cannot figure
// them out on its own.
//
// SYNTAX:
//   &'a T           reference with lifetime 'a
//   &'a mut T       mutable ref with lifetime 'a
//
// LIFETIME ELISION RULES (compiler applies automatically):
//   1. Each parameter that is a ref gets its own lifetime.
//   2. If there is exactly one input lifetime, it is assigned
//      to all output lifetimes.
//   3. If there is &self or &mut self, its lifetime is
//      assigned to all output lifetimes.
//
// 'static is a special lifetime meaning "lives for the entire
// program". String literals have type &'static str.
// ============================================================

fn main() {
    // ---------- WITHOUT LIFETIME ANNOTATION ----------
    let s1 = String::from("long string");
    let result;
    {
        let s2 = String::from("short");
        result = longest(&s1, &s2);
        println!("longest = {}", result);   // used inside inner scope
    }

    // This would NOT compile:
    // let result;
    // {
    //     let s2 = String::from("short");
    //     result = longest(&s1, &s2);
    // }
    // println!("{}", result);        // s2 already dropped

    // ---------- STRUCT HOLDING REFERENCES ----------
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first = novel.split('.').next().expect("no .");
    let excerpt = Excerpt { part: first };
    println!("excerpt: {}", excerpt.part);
    println!("announce: {}", excerpt.announce_and_return("hear!"));

    // ---------- 'static LIFETIME ----------
    let s: &'static str = "I live for the whole program";
    println!("{}", s);
}

// ---------- EXPLICIT LIFETIME ANNOTATION ----------
// Says: both inputs and output share the SAME lifetime 'a.
// The returned reference will be valid as long as BOTH inputs
// are valid.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// ---------- STRUCTS WITH REFERENCES need lifetimes ----------
struct Excerpt<'a> {
    part: &'a str,      // Excerpt cannot outlive the str it references
}

// Methods on structs with lifetimes
impl<'a> Excerpt<'a> {
    // elision rule 3: output gets self's lifetime
    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("Attention! {}", announcement);
        self.part
    }
}

// ---------- GENERICS + BOUNDS + LIFETIMES ----------
use std::fmt::Display;

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() { x } else { y }
}
