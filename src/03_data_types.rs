// ============================================================
// 03 - SCALAR DATA TYPES
// ============================================================
// Rust is statically typed. Every value has a type known at
// compile time. The compiler usually infers types, but you can
// always annotate them explicitly with `: Type`.
//
// SCALAR TYPES (single value):
//   - Integers: signed (i8..i128, isize) and unsigned (u8..u128, usize)
//   - Floats:   f32, f64 (default)
//   - Boolean:  bool (true / false)
//   - Char:     a 4-byte Unicode scalar value (NOT a byte!)
//
// `isize` / `usize` are pointer-sized: 64 bits on 64-bit systems.
//
// INTEGER LITERALS:
//   decimal:   98_222
//   hex:       0xff
//   octal:     0o77
//   binary:    0b1111_0000
//   byte:      b'A'        (u8 only)
//   typed:     42i64, 3.14f32
//
// OVERFLOW:
//   - Debug build: panics on overflow.
//   - Release build: wraps (two's complement). Use checked_/wrapping_/
//     saturating_/overflowing_ methods to be explicit.
// ============================================================

fn main() {
    // ---------- INTEGERS ----------
    let a: i8   = -128;
    let b: u8   = 255;
    let c: i32  = -2_147_483_648;
    let d: u32  = 4_000_000_000;
    let e: i64  = 9_000_000_000_000_000_000;
    let f: usize = 42;          // for indexing collections
    println!("ints: {} {} {} {} {} {}", a, b, c, d, e, f);

    // different bases produce the same value
    let dec = 255;
    let hex = 0xff;
    let oct = 0o377;
    let bin = 0b1111_1111;
    let byt = b'A';             // 65
    println!("{} {} {} {} {}", dec, hex, oct, bin, byt);

    // ---------- INTEGER ARITHMETIC ----------
    let sum    = 5 + 10;
    let diff   = 95.5 - 4.3;        // floats
    let prod   = 4 * 30;
    let quot   = 56 / 32;           // integer div: truncates -> 1
    let fquot  = 56.0 / 32.0;       // float div          -> 1.75
    let rem    = 43 % 5;            // remainder
    println!("{} {} {} {} {} {}", sum, diff, prod, quot, fquot, rem);

    // checked arithmetic to handle overflow safely
    let safe: Option<u8> = 200u8.checked_add(100);   // None (would overflow)
    println!("checked_add: {:?}", safe);

    // ---------- FLOATS ----------
    let pi: f64 = 3.141592653589793;
    let small: f32 = 1.5e-3;
    println!("pi = {}, small = {}", pi, small);

    // ---------- BOOL ----------
    let t: bool = true;
    let f: bool = false;
    println!("t = {}, f = {}", t, f);
    println!("t && f = {}, t || f = {}, !t = {}", t && f, t || f, !t);

    // ---------- CHAR ----------
    // A char is 4 bytes (a Unicode scalar value), single quotes.
    let letter: char = 'R';
    let emoji:  char = '🦀';
    let kana:   char = 'あ';
    println!("char: {} {} {}", letter, emoji, kana);

    // ---------- TYPE CASTING WITH `as` ----------
    let big: i32 = 1000;
    let small: u8 = big as u8;      // truncates: 1000 % 256 = 232
    let pi_int = 3.7_f64 as i32;    // truncates toward zero -> 3
    println!("cast: {} {}", small, pi_int);

    // ---------- TYPE INFERENCE LIMITS ----------
    // `parse` needs a hint, because it could return many numeric types
    let n: i32 = "42".parse().unwrap();
    let m     = "42".parse::<f64>().unwrap();   // turbofish ::<>
    println!("parsed: {} {}", n, m);

    // ---------- MIN / MAX CONSTANTS ----------
    println!("i32 range: {} to {}", i32::MIN, i32::MAX);
    println!("u8  range: {} to {}", u8::MIN, u8::MAX);
}
