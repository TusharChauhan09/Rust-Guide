// ============================================================
// 21 - LIFETIMES
// ============================================================
// Every reference in Rust has a lifetime: the scope for which
// the reference is valid. Usually inferred by the compiler.
// You only write lifetimes when the compiler cannot figure
// them out on its own.
//
// Lifetimes are used to:
// Describe how the lifetime of the output reference is related to the lifetimes of the input references
//
// IMPORTANT:
// Lifetimes do NOT:
//   ❌ extend how long data lives
//   ❌ control memory allocation
//   ❌ change runtime behavior
//   ✔ they are compile-time checks only
//
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
    // ---------- WITHOUT LIFETIME ANNOTATION But Ownership taken ----------
    let s1 = String::from("long string");
    let result;
    {
        let s2 = String::from("short");
        result = longestOWNERSHIP(s1, s2);  // ! takes the ownership 
    }
    println!("longest = {}", result);   // used inside outside scope
    // this will work as we have taken the ownership and given it to the longestOWNERSHIP function and so it can return use the answer  as compiler can figure out that the ownership of s1 and s2 is now with longestOWNERSHIP function and so it can return the longest string without any issues. But this is not what we want as we want to return a reference to the longest string and not take the ownership of it. This is where lifetimes come in to tell the compiler that the reference we are returning is valid for as long as both s1 and s2 are valid.

    // ---------- WITHOUT LIFETIME ANNOTATION ----------
    let s1 = String::from("long string");
    let result;
    {
        let s2 = String::from("short");
        result = longest(&s1, &s2);  // ! NOTE :  ref 
        println!("longest = {}", result);   // used inside inner scope
    }

    // This would NOT compile:
    // let result;
    // {
    //     let s2 = String::from("short");
    //     result = longest(&s1, &s2);
    // }
    // println!("{}", result);        // s2 already dropped

    // here we are outside of the scope of s2 and so the reference to s2 is no longer valid and so the compiler will not allow us to use result as it may be referencing s2 which is already dropped. This is where we need lifetime annotations to tell the compiler that the reference in result is valid for as long as both s1 and s2 are valid.

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


// ownership taken
fn longestOWNERSHIP(x: String, y: String) -> String {
    if x.len() > y.len() { x } else { y }
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


// Note : 

// ! Case 1: Struct (User)

// struct User<'a, 'b> {
//     first_name: &'a str,
//     last_name: &'b str,
// }

// let first_name = String::from("Harkirat");
// {
//     let last_name = String::from("Singh");
//     let user = User {
//         first_name: &first_name, // lives long
//         last_name: &last_name,   // lives short
//     };
// }

// 👉 You STORE both values
// User has:
// - first_name
// - last_name
// ✔ They are independent
// ✔ So lifetimes can be different
// ➡️ 'a, 'b

// ! 2: Function (longest)

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str

// let s1 = String::from("long string"); let result; { let s2 = String::from("short"); result = longest(&s1, &s2);


// 👉 You RETURN one value
// return either x OR y
// ❗ Rust doesn’t know which one
// ➡️ So both must be equally safe
// ➡️ same lifetime 'a

// ! So
// Use 'a, 'b → when references are independent
// Use 'a → when return depends on inputs
