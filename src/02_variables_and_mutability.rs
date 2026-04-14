// ============================================================
// 02 - VARIABLES & MUTABILITY
// ============================================================
// In Rust, variables are IMMUTABLE by default. This is a core
// safety feature: you opt-in to mutation with `mut`.
//
// THREE WAYS TO BIND A NAME:
//   let x = 5;          immutable
//   let mut x = 5;      mutable
//   const X: i32 = 5;   compile-time constant (always typed)
//
// SHADOWING:
//   `let` can re-declare a name. The new binding REPLACES the
//   old one and may even change its type. This is different
//   from `mut`, which only changes the value, not the type.
//
// CONSTANTS vs `let`:
//   - `const` must have an explicit type annotation.
//   - `const` can live at any scope, including module level.
//   - `const` value is computed at compile time.
//   - Convention: SCREAMING_SNAKE_CASE.
//
// `static`:
//   - Like `const`, but has a fixed memory address.
//   - Used for globals; `static mut` is unsafe.
// ============================================================

const MAX_POINTS: u32 = 100_000;        // numeric literals can use _
static GREETING: &str = "hello";

fn main() {
    // ---------- IMMUTABLE BY DEFAULT ----------
    let x = 5;
    println!("x = {}", x);
    // x = 6; // ERROR: cannot assign twice to immutable variable

    // ---------- MUTABLE WITH `mut` ----------
    let mut y = 10;
    println!("y = {}", y);
    y = 20;
    println!("y = {}", y);

    // ---------- CONSTANTS ----------
    println!("MAX_POINTS = {}", MAX_POINTS);
    println!("GREETING   = {}", GREETING);

    // ---------- SHADOWING ----------
    let z = 5;
    let z = z + 1;          // new binding, value is 6
    let z = z * 2;          // new binding, value is 12
    println!("z = {}", z);

    // shadowing CAN change the type:
    let spaces = "   ";
    let spaces = spaces.len();   // now an integer
    println!("spaces = {}", spaces);

    // mut CANNOT change the type:
    let mut n = 5;
    n = 6;
    // n = "six"; // ERROR: expected integer, found &str

    // ---------- TYPE INFERENCE & ANNOTATIONS ----------
    let inferred = 42;              // i32 by default
    let annotated: u64 = 42;        // explicit type
    let parsed: i32 = "42".parse().expect("not a number");
    println!("{} {} {}", inferred, annotated, parsed);

    // ---------- BLOCK SCOPE ----------
    let outer = 1;
    {
        let inner = 2;
        println!("inside block: outer = {}, inner = {}", outer, inner);
    }
    // println!("{}", inner); // ERROR: not in scope

    // ---------- BLOCKS ARE EXPRESSIONS ----------
    // The last line without `;` is the value of the block.
    let value = {
        let a = 3;
        let b = 4;
        a + b           // no semicolon -> this is the result
    };
    println!("value from block = {}", value);
}
