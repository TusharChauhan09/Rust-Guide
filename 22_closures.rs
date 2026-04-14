// ============================================================
// 22 - CLOSURES
// ============================================================
// Closures are anonymous functions that can capture variables
// from their environment.
//
// SYNTAX:
//   |x, y| x + y                      // inferred types
//   |x: i32, y: i32| -> i32 { x + y } // fully typed
//
// CAPTURE MODES (chosen automatically, can be forced):
//   - By reference    -> Fn
//   - By mutable ref  -> FnMut
//   - By value (move) -> FnOnce (consumed) / Fn if captured is Copy
//
// TRAIT HIERARCHY:
//   Fn      can be called many times, borrows immutably
//   FnMut   can be called many times, borrows mutably
//   FnOnce  can be called ONCE; may consume captured values
// ============================================================

fn main() {
    // ---------- BASIC CLOSURE ----------
    let add = |a, b| a + b;
    println!("{}", add(2, 3));

    // With explicit types
    let multiply = |a: i32, b: i32| -> i32 { a * b };
    println!("{}", multiply(4, 5));

    // ---------- CAPTURING ENVIRONMENT ----------
    let x = 10;
    let add_x = |n| n + x;          // captures x by reference
    println!("{}", add_x(5));

    let mut count = 0;
    let mut increment = || {
        count += 1;                 // captures by mutable ref
        println!("count = {}", count);
    };
    increment();
    increment();

    // ---------- MOVE KEYWORD ----------
    let data = vec![1, 2, 3];
    let owns = move || println!("owns: {:?}", data);
    owns();
    // println!("{:?}", data);      // ERROR: moved

    // ---------- CLOSURES AS PARAMETERS ----------
    apply(|x| x * 2);
    apply(square);                  // regular fn pointer works too

    // Fn vs FnMut vs FnOnce
    do_twice(|| println!("tick"));

    let s = String::from("taken");
    consume_once(move || println!("got {}", s));

    // ---------- RETURNING CLOSURES ----------
    let adder = make_adder(5);
    println!("{}", adder(10));      // 15

    // ---------- CLOSURES WITH ITERATORS ----------
    let nums = vec![1, 2, 3, 4, 5];
    let squares: Vec<i32> = nums.iter().map(|x| x * x).collect();
    let evens: Vec<i32> = nums.iter().filter(|&&x| x % 2 == 0).copied().collect();
    println!("squares = {:?}", squares);
    println!("evens = {:?}", evens);
}

// impl Fn(i32) -> i32  means: any callable taking i32, returning i32
fn apply<F: Fn(i32) -> i32>(f: F) {
    println!("apply -> {}", f(7));
}

fn square(x: i32) -> i32 { x * x }

fn do_twice<F: Fn()>(f: F) {
    f();
    f();
}

fn consume_once<F: FnOnce()>(f: F) {
    f();
    // f();   // ERROR: FnOnce can only be called once
}

// Boxed return because each closure has a different type
fn make_adder(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |n| n + x)
}
