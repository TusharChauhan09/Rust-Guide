// ============================================================
// 05 - FUNCTIONS
// ============================================================
// Functions are declared with `fn`. Parameters and the return
// type MUST be annotated.
//
// SYNTAX:
//   fn name(param: Type, ...) -> ReturnType { body }
//
// EXPRESSIONS vs STATEMENTS:
//   - A STATEMENT performs an action (`let x = 5;`) and returns ()
//   - An EXPRESSION evaluates to a value (`5 + 3`, `if cond { a } else { b }`)
//   - The LAST expression in a block (no `;`) is the block's value.
//   - Adding `;` turns an expression into a statement -> () is returned.
//
// RETURNING:
//   - `return x;` is allowed but un-idiomatic.
//   - The idiomatic style is to leave the final expression bare.
//
// NAMING:
//   - functions and variables: snake_case
//   - types and traits:        CamelCase
//   - constants and statics:   SCREAMING_SNAKE_CASE
// ============================================================

fn main() {
    greet("Tushar");

    let s = add(3, 4);
    println!("3 + 4 = {}", s);

    // expression returning a value
    let abs = absolute(-7);
    println!("|-7| = {}", abs);

    // multiple return values via a tuple
    let (lo, hi) = min_max(3, 9);
    println!("min={}, max={}", lo, hi);

    // a function with no return value implicitly returns ()
    let unit_value = greet("again");
    println!("greet returned: {:?}", unit_value);

    // higher-level: passing functions as arguments (see file 22 for closures)
    let result = apply_twice(square, 3);
    println!("apply_twice(square, 3) = {}", result);

    // diverging function (returns `!`, the never type)
    // never_returns();   // would terminate the program

    // nested function (scoped to main)
    fn local_doubled(x: i32) -> i32 { x * 2 }
    println!("local: {}", local_doubled(21));
}

// ---------- NO RETURN VALUE ----------
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// ---------- SIMPLE RETURN ----------
fn add(a: i32, b: i32) -> i32 {
    a + b               // no semicolon -> this is the return value
}

// ---------- EARLY RETURN ----------
fn absolute(n: i32) -> i32 {
    if n < 0 {
        return -n;      // explicit early return
    }
    n
}

// ---------- MULTIPLE RETURNS VIA TUPLE ----------
fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b { (a, b) } else { (b, a) }
}

// ---------- FUNCTION POINTER PARAMETER ----------
// `fn(i32) -> i32` is the type of a plain function pointer.
fn apply_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn square(x: i32) -> i32 { x * x }

// ---------- DIVERGING FUNCTION ----------
// The `!` "never" type means this function never returns normally
// (it panics, loops forever, exits, etc.).
#[allow(dead_code)]
fn never_returns() -> ! {
    panic!("this function never returns");
}
