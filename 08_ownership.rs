// ============================================================
// 08 - OWNERSHIP
// ============================================================
// Ownership is Rust's most unique feature. It enables memory
// safety without a garbage collector.
//
// THE THREE RULES OF OWNERSHIP:
//   1. Each value in Rust has an owner (a variable).
//   2. There can only be ONE owner at a time.
//   3. When the owner goes out of scope, the value is dropped.
//
// STACK vs HEAP:
//   - Stack: fixed-size data (integers, bools, chars). Fast.
//   - Heap: dynamic-size data (String, Vec). Slower; needs
//     allocation and deallocation.
//
// MOVE vs COPY:
//   - Types with the Copy trait (i32, bool, f64, char, tuples
//     of Copy types) are COPIED on assignment.
//   - Heap types (String, Vec) are MOVED; the old variable
//     becomes invalid.
// ============================================================

fn main() {
    // ---------- COPY (stack data) ----------
    let x = 5;
    let y = x;      // x is copied, both are valid
    println!("x = {}, y = {}", x, y);

    // ---------- MOVE (heap data) ----------
    let s1 = String::from("hello");
    let s2 = s1;    // s1 is MOVED into s2; s1 is no longer valid
    // println!("{}", s1); // ERROR: borrow of moved value
    println!("s2 = {}", s2);

    // ---------- CLONE (deep copy) ----------
    let a = String::from("world");
    let b = a.clone();  // deep copy on heap; both valid
    println!("a = {}, b = {}", a, b);

    // ---------- OWNERSHIP AND FUNCTIONS ----------
    let s = String::from("taken");
    takes_ownership(s);       // s is moved into function
    // println!("{}", s);      // ERROR: s no longer valid

    let n = 10;
    makes_copy(n);            // n is copied
    println!("n still valid: {}", n);

    // ---------- RETURNING OWNERSHIP ----------
    let s3 = gives_ownership();
    println!("got: {}", s3);

    let s4 = String::from("hello");
    let s5 = takes_and_gives_back(s4);
    println!("returned: {}", s5);

    // ---------- SCOPE ----------
    {
        let scoped = String::from("temp");
        println!("{}", scoped);
    } // `scoped` is dropped here; memory freed automatically
}

fn takes_ownership(s: String) {
    println!("inside fn: {}", s);
} // s goes out of scope and is dropped

fn makes_copy(n: i32) {
    println!("inside fn: {}", n);
} // n goes out of scope; nothing special happens

fn gives_ownership() -> String {
    String::from("new string")
}

fn takes_and_gives_back(s: String) -> String {
    s
}
