// ============================================================
// 01 - HELLO WORLD, COMMENTS & I/O
// ============================================================
// Every Rust program starts at `fn main()`.
// `println!` is a MACRO (note the `!`), not a function.
// It prints a line to standard output.
//
// PRINT MACROS:
//   println!  -> print with a trailing newline
//   print!    -> print without a newline
//   eprintln! -> print to standard ERROR
//   format!   -> build a String instead of printing
//
// FORMAT PLACEHOLDERS:
//   {}     -> Display (user-friendly)
//   {:?}   -> Debug   (developer view, requires Debug trait)
//   {:#?}  -> pretty-printed Debug
//   {0} {1} -> positional
//   {name} -> named
//
// COMMENTS:
//   // line comment
//   /* block comment */
//   /// doc comment for the item below it
//   //! doc comment for the enclosing item (file/module)
// ============================================================

use std::io::{self, Write};

fn main() {
    // ---------- BASIC PRINTING ----------
    println!("Hello, world!");
    print!("no newline ");
    print!("on this line\n");

    // ---------- FORMAT PLACEHOLDERS ----------
    let name = "Ada";
    let age = 36;
    println!("{} is {} years old", name, age);     // Display
    println!("{0} {0} {1}", name, age);            // positional
    println!("{n} is {a}", n = name, a = age);     // named
    println!("{:?}", (1, 2, 3));                   // Debug
    println!("{:#?}", vec![1, 2, 3]);              // pretty Debug

    // ---------- NUMERIC FORMATTING ----------
    println!("{:5}", 42);       // width 5, right-aligned   -> "   42"
    println!("{:<5}|", 42);     // left-aligned             -> "42   |"
    println!("{:05}", 42);      // zero-padded              -> "00042"
    println!("{:.3}", 3.14159); // 3 decimals               -> "3.142"
    println!("{:b}", 10);       // binary                   -> "1010"
    println!("{:x}", 255);      // hex                      -> "ff"

    // ---------- READING INPUT ----------
    print!("Type something: ");
    io::stdout().flush().unwrap();          // ensure prompt shows
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("failed to read");
    let line = line.trim();                 // strip trailing newline
    println!("you typed: {}", line);

    // ---------- ESCAPE SEQUENCES ----------
    println!("tab:\there");
    println!("newline:\nhere");
    println!("quote: \"hi\"");
    println!("backslash: \\");

    // ---------- format! BUILDS A STRING ----------
    let s = format!("{}-{}", name, age);
    println!("built: {}", s);
}
