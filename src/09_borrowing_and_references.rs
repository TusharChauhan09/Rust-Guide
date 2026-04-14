// ============================================================
// 09 - BORROWING AND REFERENCES
// ============================================================
// Instead of transferring ownership, we can "borrow" a value
// using references (&).
//
// REFERENCE SYNTAX:
//   &T   -> shared (immutable) reference
//   &mut T -> mutable reference
//
// BORROWING RULES (enforced at compile time):
//   1. At any time, you can have EITHER:
//        - any number of immutable references (&T), OR
//        - exactly one mutable reference (&mut T).
//   2. References must always be valid (no dangling refs).
//
// These rules prevent data races at compile time.
// ============================================================

fn main() {
    // ---------- IMMUTABLE REFERENCE ----------
    let s1 = String::from("hello");
    let len = calculate_length(&s1);   // pass a reference
    println!("'{}' has length {}", s1, len);   // s1 still valid

    // ---------- MUTABLE REFERENCE ----------
    let mut s = String::from("hello");
    change(&mut s);
    println!("after change: {}", s);

    // ---------- MULTIPLE IMMUTABLE REFS OK ----------
    let s2 = String::from("rust");
    let r1 = &s2;
    let r2 = &s2;
    println!("{} and {}", r1, r2);

    // ---------- ONLY ONE MUTABLE REF AT A TIME ----------
    let mut s3 = String::from("a");
    {
        let r1 = &mut s3;
        r1.push_str("b");
    } // r1 goes out of scope here
    let r2 = &mut s3;   // now this is fine
    r2.push_str("c");
    println!("{}", s3);

    // ---------- CANNOT MIX MUTABLE AND IMMUTABLE ----------
    let mut s4 = String::from("mix");
    let r1 = &s4;
    let r2 = &s4;
    println!("{} {}", r1, r2);     // last use of r1, r2
    let r3 = &mut s4;              // now mutable ref is OK (NLL)
    r3.push('!');
    println!("{}", r3);

    // ---------- DANGLING REFERENCES (prevented) ----------
    // fn dangle() -> &String {            // would not compile
    //     let s = String::from("hi");
    //     &s                              // s dropped, ref invalid
    // }
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but since it was borrowed, nothing dropped

fn change(s: &mut String) {
    s.push_str(", world");
}
