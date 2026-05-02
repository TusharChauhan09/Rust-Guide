// ============================================================
// 27 - MACROS
// ============================================================
// Macros are code that writes code (metaprogramming). Rust has:
// macros are a powerful feature that allows for metaprogramming by enabling 
// the generation of code at compile-time. Macros in Rust are similar to functions 
// but differ in that they operate at the syntactic level—they generate or transform
// Rust code before the program is actually compiled.
//
//   1. DECLARATIVE MACROS   (macro_rules!)       <- most common
//   2. PROCEDURAL MACROS    (compile-time funcs that transform
//                            TokenStreams — need a separate crate)
//        - custom derive:  #[derive(MyTrait)] :  generates trait impls
//        - attribute-like: #[my_attr] :  modifies items (functions, structs, etc.)
//        - function-like:  my_macro!(...) :  looks like a function call but is a macro
//
// MACROS vs FUNCTIONS:
//   - Macros expand at compile time; functions run at runtime.
//   - Macros can take variable numbers of args (e.g. println!).
//   - Macros can generate type definitions, impl blocks, etc.
// ============================================================

// ---------- BUILT-IN MACROS YOU ALREADY USE ----------
// println!, print!, eprintln!, format!, write!, writeln!,
// vec!, assert!, assert_eq!, assert_ne!, panic!, dbg!, todo!,
// unimplemented!, include_str!, env!, cfg!, matches!

// ---------- BASIC macro_rules! ----------
// Takes 0 arguments; prints a greeting.
macro_rules! say_hello {
    () => {
        println!("Hello, macro!");
    };
}

// Takes exactly one expression
macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

// Fragment specifiers commonly used:
//   expr  -> expression
//   ident -> identifier
//   ty    -> type
//   pat   -> pattern
//   stmt  -> statement
//   block -> { ... }
//   path  -> a::b::c

// ---------- MULTIPLE PATTERNS ----------
macro_rules! max {
    ($a:expr, $b:expr) => {
        if $a > $b { $a } else { $b }
    };
    // Recursive: max!(a, b, c, d, ...)
    ($a:expr, $($rest:expr),+) => {
        max!($a, max!($($rest),+))
    };
}

// ---------- VARIADIC MACRO (like vec!) ----------
macro_rules! my_vec {
    ( $( $x:expr ),* $(,)? ) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )*
            v
        }
    };
}

// ---------- GENERATING CODE ----------
macro_rules! create_getters {
    ($struct_name:ident, $($field:ident : $t:ty),+ $(,)?) => {
        struct $struct_name {
            $( pub $field: $t, )+
        }
        impl $struct_name {
            $(
                pub fn $field(&self) -> &$t {
                    &self.$field
                }
            )+
        }
    };
}
create_getters!(Person, name: String, age: u32);


// ? Procedual macro
// ! 1. custom derive: :  #[derive(MyTrait)] :  generates trait impls
#[derive(Debug , PartialEq)]  // for printing the using debug {:?}
struct Rectangle {
    width: u32,
    height: u32,
}

// display trait for custom formatting with {} to print the rectangle
// println!("r = {}", r);  // uses Display trait
use std::fmt::{self, Display, Formatter};
impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangle {{ width: {}, height: {} }}", self.width, self.height)
    }
}

// we we dont use allready derive PartialEq, we can implement it ourselves like this
// impl PartialEq for Rectangle {
//     fn eq(&self, other: &Self) -> bool {
//         self.width == other.width && self.height == other.height
//     }
// }

// ! 2. attribute-like: #[my_attr] :  modifies items (functions, structs, etc.)
#[route("GET")]
fn home() {
    println!("Welcome to the home page!");
}

#[route("POST")]
fn create_post() {
    println!("Creating a new post!");
}

// ! 3. function-like:  my_macro!(...) :  looks like a function call but is a macro



// ---------- dbg! AS DEBUGGING HELPER ----------
// dbg!(expr) prints file, line, expr, and its value, and returns it.

fn main() {
    say_hello!();

    let s = square!(5);
    // ? 1. Declarative
    println!("square(5) = {}", s);

    println!("max = {}", max!(3, 1, 4, 1, 5, 9, 2, 6));

    let v = my_vec![10, 20, 30];
    println!("{:?}", v);

    let p = Person { name: String::from("Ada"), age: 36 };
    println!("{} ({})", p.name(), p.age());

    // dbg! in action
    let x = dbg!(2 + 3);       // prints "[file:line] 2 + 3 = 5"
    println!("x = {}", x);

    // assert! variants
    assert!(1 + 1 == 2);
    assert_eq!(3 * 3, 9);
    assert_ne!(1, 2);

    // matches! macro
    let opt = Some(5);
    let is_some_big = matches!(opt, Some(n) if n > 3);
    println!("is big: {}", is_some_big);


    // ! 1. custom derive
    let r = Rectangle { width: 30, height: 50 };
    println!("{:?}",r); // uses Debug trait
    println!("r = {}", r);  // uses Display trait

    let r2 = Rectangle { width: 30, height: 50 };
    println!("r == r2: {}", r == r2);  // uses PartialEq
}




// ---------- PROCEDURAL MACROS (sketch) ----------
// Proc macros must live in a crate with `proc-macro = true`:
//
//   // in Cargo.toml of proc-macro crate:
//   [lib]
//   proc-macro = true
//
//   // in lib.rs:
//   use proc_macro::TokenStream;
//   #[proc_macro_derive(HelloMacro)]
//   pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
//       // parse, generate code, return tokens
//   }
//
// Popular crates using proc macros: serde, tokio, clap, thiserror.
