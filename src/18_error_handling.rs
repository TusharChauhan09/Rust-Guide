// ============================================================
// 18 - ERROR HANDLING
// ============================================================
// Rust splits errors into two categories:
//
//   Unrecoverable -> panic!()       -> program aborts
//   Recoverable   -> Result<T, E>   -> caller decides
//
// Rust does NOT have exceptions. Errors are values.
//
// THE ? OPERATOR:
//   expr?  ->  if Ok(v) then v, if Err(e) then return Err(e.into())
//   Works on Result and Option in matching function signatures.
// ============================================================

// how can we handle errors in Rust
// 1. panic!() - unrecoverable error, aborts the thread
// 2. Result<T, E> - recoverable error, caller decides what to do 
// 3. ? operator - propagates errors up the call stack, converting types as needed
// 4. .unwrap() / .expect("message") - panic on Err, with optional message


use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

fn main() {
    // ---------- PANIC ----------
    // panic!("something broke");        // aborts the thread
    // let v = vec![1, 2, 3];
    // let _ = v[99];                    // also panics (OOB)

    // ---------- BASIC Result ----------
    let file = File::open("not_a_file.txt");
    match file {
        Ok(f) => println!("opened: {:?}", f),
        Err(e) => println!("failed: {}", e),
    }

    // unwrap / expect
    // File::open("x").unwrap();                  // panic on Err
    // File::open("x").expect("open failed");    // panic w/ message

    // ---------- PROPAGATING WITH ? ----------
    match read_username() {
        Ok(u) => println!("user: {}", u),
        Err(e) => println!("read err: {}", e),
    }

    // ? on Option
    let first = first_char("hello");
    println!("{:?}", first);

    // ---------- CUSTOM ERROR TYPE ----------
    match double_from_str("12") {
        Ok(n) => println!("double = {}", n),
        Err(e) => println!("error: {:?}", e),
    }
    match double_from_str("oops") {
        Ok(n) => println!("double = {}", n),
        Err(e) => println!("error: {:?}", e),
    }
}

// ---------- ? WITH io::Error ----------
fn read_username() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;    // early return on Err
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
    // shorter: std::fs::read_to_string("hello.txt")
}

// ---------- ? WITH Option ----------
fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}

// ---------- DEFINING A CUSTOM ERROR ----------
#[derive(Debug)]
enum MyError {
    Parse(ParseIntError),
    TooLarge,
}

// Implement From so ? can convert automatically
impl From<ParseIntError> for MyError {
    fn from(e: ParseIntError) -> Self {
        MyError::Parse(e)
    }
}

fn double_from_str(s: &str) -> Result<i32, MyError> {
    let n: i32 = s.parse()?;            // ParseIntError -> MyError
    if n > 1_000_000 {
        return Err(MyError::TooLarge);
    }
    Ok(n * 2)
}
