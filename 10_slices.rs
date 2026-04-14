// ============================================================
// 10 - SLICES
// ============================================================
// A slice is a reference to a contiguous sequence of elements
// in a collection, rather than the whole collection. Slices
// do NOT own the data they reference.
//
// SYNTAX:
//   &s[start..end]    // start inclusive, end exclusive
//   &s[..end]         // from 0
//   &s[start..]       // to the end
//   &s[..]            // full slice
//
// COMMON SLICE TYPES:
//   &str         -> string slice (a slice of a String)
//   &[T]         -> slice of any type
// ============================================================

fn main() {
    // ---------- STRING SLICES ----------
    let s = String::from("hello world");
    let hello = &s[0..5];       // "hello"
    let world = &s[6..11];      // "world"
    let whole = &s[..];         // full slice
    println!("{} | {} | {}", hello, world, whole);

    // ---------- STRING LITERALS ARE SLICES ----------
    let literal: &str = "I am a string slice";
    println!("{}", literal);

    // ---------- FUNCTIONS TAKING &str IS PREFERRED ----------
    // Accepts both &String and &str (via deref coercion)
    let word = first_word(&s);
    println!("first word = {}", word);

    let literal = "hello rust";
    let word2 = first_word(literal);
    println!("first word = {}", word2);

    // ---------- ARRAY SLICES ----------
    let arr = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];   // [2, 3, 4]
    println!("slice = {:?}", slice);

    // ---------- ITERATING OVER A SLICE ----------
    for n in slice {
        print!("{} ", n);
    }
    println!();

    // ---------- SLICES PREVENT BUGS ----------
    // If you hold a slice and try to mutate the source:
    // let mut s2 = String::from("hi");
    // let r = &s2[..];
    // s2.clear();            // ERROR: mutable borrow while immutable exists
    // println!("{}", r);
}

// Returns a slice of the first word in the string.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
