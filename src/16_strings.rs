// ============================================================
// 16 - STRINGS
// ============================================================
// Rust has two main string types:
//   String   - owned, heap-allocated, growable, UTF-8
//   &str     - borrowed slice, fixed-size view into UTF-8 data
//
// String literals ("hi") are of type &'static str. it is store in the binary and points to there it is just hard coded in the binary. It is immutable and has 'static lifetime (lives for the entire duration of the program).
// ============================================================
// STRINGS ARE UTF-8. You CANNOT index by integer because
// bytes != characters != grapheme clusters.
// ============================================================

fn main() {
    // ---------- CREATING STRINGS ----------
    let s1 = String::new();                 // empty
    let s2 = String::from("hello");
    let s3 = "hello".to_string();
    let s4: String = "hi".into();
    println!("{} {} {} {}", s1, s2, s3, s4);

    // ---------- APPENDING ----------
    let mut s = String::from("Hello");
    s.push_str(", ");        // add &str
    s.push('w');             // add single char
    s.push_str("orld!");
    println!("{}", s);

    // ---------- CONCATENATION ----------
    let a = String::from("foo");
    let b = String::from("bar");
    let c = a + &b;          // a moved; b borrowed
    println!("{}", c);
    // println!("{}", a);    // ERROR: a moved

    // format! does NOT take ownership
    let x = String::from("tic");
    let y = String::from("tac");
    let z = String::from("toe");
    let s = format!("{}-{}-{}", x, y, z);
    println!("{} (originals: {} {} {})", s, x, y, z);

    // ---------- LENGTH (in BYTES, not chars) ----------
    let hello = String::from("Здравствуйте");
    println!("bytes: {}", hello.len());          // 24, not 12

    // ---------- ITERATING ----------
    for c in "नमस्ते".chars() {      // over Unicode scalar values
        print!("{} ", c);
    }
    println!();

    for b in "abc".bytes() {
        print!("{} ", b);
    }
    println!();

    // ---------- SLICING (must be on char boundary) ----------
    let s = String::from("hello");
    let hi = &s[0..2];         // "he"
    println!("{}", hi);

    // ---------- COMMON METHODS ----------
    let text = String::from("  Rust Lang  ");
    println!("trim: '{}'", text.trim());
    println!("upper: {}", text.to_uppercase());
    println!("lower: {}", text.to_lowercase());
    println!("contains 'Rust': {}", text.contains("Rust"));
    println!("replace: {}", text.replace("Rust", "Go"));

    let csv = "a,b,c,d";
    let parts: Vec<&str> = csv.split(',').collect();
    println!("{:?}", parts);

    // parse string to number
    let n: i32 = "42".parse().unwrap();
    println!("parsed: {}", n);

    // &str vs String function params: prefer &str
    print_str("literal");
    print_str(&String::from("owned"));
}

fn print_str(s: &str) {
    println!("got: {}", s);
}
