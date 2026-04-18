// ============================================================
// 14 - OPTION AND RESULT
// ============================================================
// Rust has NO null. Use Option<T> and Result<T, E> instead.
//
// Option<T>:
//   enum Option<T> { Some(T), None }
//   Used when a value might be absent.
//
// Result<T, E>:
//   enum Result<T, E> { Ok(T), Err(E) }
//   Used when an operation can fail.
//
// Both are in the prelude, so Some/None/Ok/Err are available
// without imports.
// ============================================================

use std::num::ParseIntError;

fn main() {
    // ---------- OPTION BASICS ----------
    let some_num: Option<i32> = Some(5);
    let no_num: Option<i32> = None;

    // unwrap: panics if None  (avoid in production)
    println!("{}", some_num.unwrap());

    // unwrap_or: default if None
    println!("{}", no_num.unwrap_or(0));

    // unwrap_or_else: default from closure
    println!("{}", no_num.unwrap_or_else(|| 42));

    // expect: unwrap with custom panic msg
    // println!("{}", no_num.expect("needed a value"));

    // map: transform Some, preserve None
    let doubled = some_num.map(|x| x * 2);
    println!("{:?}", doubled);   // Some(10)

    // and_then (flatMap): chain Options
    let maybe = Some(4).and_then(|x| if x > 0 { Some(x * x) } else { None });
    println!("{:?}", maybe);

    // is_some / is_none
    println!("has value? {}", some_num.is_some());

    // ---------- MATCHING OPTION ----------
    match divide(10.0, 2.0) {
        Some(v) => println!("result = {}", v),
        None => println!("cannot divide by zero"),
    }

    // ---------- RESULT BASICS ----------
    let good: Result<i32, ParseIntError> = "42".parse();
    let bad: Result<i32, ParseIntError> = "abc".parse();

    match good {
        Ok(n) => println!("parsed {}", n),
        Err(e) => println!("failed: {}", e),
    }

    println!("unwrap_or: {}", bad.unwrap_or(0));

    // ---------- ? OPERATOR ----------
    // ? propagates errors: returns early with the Err if the
    // value is Err, otherwise unwraps the Ok.
    match parse_and_double("21") {
        Ok(n) => println!("double = {}", n),
        Err(e) => println!("error: {}", e),
    }
    match parse_and_double("oops") {
        Ok(n) => println!("double = {}", n),
        Err(e) => println!("error: {}", e),
    }

    // ---------- CONVERTING BETWEEN Option & Result ----------
    let opt: Option<i32> = Some(7);
    let res: Result<i32, &str> = opt.ok_or("missing");
    println!("{:?}", res);

    let res2: Result<i32, &str> = Err("bad");
    let opt2: Option<i32> = res2.ok();
    println!("{:?}", opt2);
}

// ? propagates parse error automatically
fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    let n: i32 = s.parse()?;    // on Err, returns early
    Ok(n * 2)
}

// these are the 2 functions we used above to demonstrate Option and Result

// Option: returns Some(result) or None if division by zero
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

// Result: returns Ok(result) or Err(error message) if division by zero
fn divide2(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 { Err("cannot divide by zero".into()) } else { Ok(a / b) }
}