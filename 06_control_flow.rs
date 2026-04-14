// ============================================================
// 06 - CONTROL FLOW (if / else)
// ============================================================
// `if` is an EXPRESSION in Rust, not just a statement. That
// means it evaluates to a value and can be used on the right
// side of `let`.
//
// RULES:
//   - The condition MUST be a `bool`. There is no implicit
//     conversion from numbers (no `if 1 { ... }`).
//   - All branches in an `if`/`else if`/`else` must produce
//     the SAME type when used as a value.
//   - Parentheses around the condition are not used.
//
// For multi-way branching on a value's shape, prefer `match`
// (covered in file 13). Use `if let` for a single-pattern
// shortcut.
// ============================================================

fn main() {
    // ---------- BASIC if / else ----------
    let n = 7;

    if n % 2 == 0 {
        println!("{} is even", n);
    } else {
        println!("{} is odd", n);
    }

    // ---------- else if CHAIN ----------
    let score = 82;
    let grade = if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else {
        'F'
    };
    println!("grade = {}", grade);

    // ---------- if AS AN EXPRESSION ----------
    let condition = true;
    let value = if condition { 5 } else { 10 };
    println!("value = {}", value);

    // both arms must have the same type:
    // let bad = if condition { 5 } else { "ten" }; // ERROR

    // ---------- COMPARISON OPERATORS ----------
    let (a, b) = (3, 5);
    println!("a == b: {}", a == b);
    println!("a != b: {}", a != b);
    println!("a <  b: {}", a <  b);
    println!("a <= b: {}", a <= b);
    println!("a >  b: {}", a >  b);
    println!("a >= b: {}", a >= b);

    // ---------- LOGICAL OPERATORS (short-circuit) ----------
    let x = 5;
    if x > 0 && x < 10 {
        println!("{} is a single digit", x);
    }
    if x == 0 || x == 5 {
        println!("{} is 0 or 5", x);
    }
    if !(x == 0) {
        println!("{} is non-zero", x);
    }

    // ---------- NESTED if ----------
    let temp = 22;
    if temp > 0 {
        if temp < 30 {
            println!("comfortable");
        } else {
            println!("hot");
        }
    } else {
        println!("freezing");
    }

    // ---------- if let (SINGLE-PATTERN MATCH SHORTCUT) ----------
    // Sneak peek of pattern matching (full coverage in file 13).
    let maybe: Option<i32> = Some(42);
    if let Some(v) = maybe {
        println!("got value: {}", v);
    } else {
        println!("nothing");
    }

    // ---------- TERNARY-STYLE ASSIGNMENT ----------
    // Rust has no `?:` ternary operator — use `if` as an expression.
    let parity = if n % 2 == 0 { "even" } else { "odd" };
    println!("{} is {}", n, parity);
}
