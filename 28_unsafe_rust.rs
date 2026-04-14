// ============================================================
// 28 - UNSAFE RUST
// ============================================================
// Most Rust code is "safe Rust" — the compiler checks memory
// safety and data-race freedom. "Unsafe Rust" lets you opt
// out of some checks when the compiler cannot prove safety
// but YOU can.
//
// WHAT unsafe UNLOCKS (5 "unsafe superpowers"):
//   1. Dereference a raw pointer.
//   2. Call an unsafe function or method.
//   3. Access or modify a mutable static variable.
//   4. Implement an unsafe trait.
//   5. Access fields of a `union`.
//
// unsafe DOES NOT turn off the borrow checker or the type
// system. It just allows the five things above.
//
// DESIGN ADVICE:
//   - Keep unsafe blocks SMALL and wrap them in SAFE APIs.
//   - Document the safety invariants callers must uphold.
// ============================================================

fn main() {
    // ---------- 1. RAW POINTERS ----------
    // Two kinds:
    //   *const T  - immutable
    //   *mut T    - mutable
    // Unlike references, they:
    //   - can be null
    //   - can ignore borrowing rules
    //   - are not guaranteed to point at valid memory
    //   - can be created in safe code, but only dereferenced in unsafe
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 = {}", *r1);
        *r2 = 10;
        println!("r2 = {}", *r2);
    }

    // ---------- 2. UNSAFE FUNCTIONS ----------
    unsafe fn dangerous() {
        println!("doing something dangerous");
    }

    unsafe {
        dangerous();      // must be called inside unsafe
    }

    // ---------- SAFE ABSTRACTION OVER UNSAFE ----------
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (a, b) = split_at_mut(&mut v, 3);
    println!("a={:?} b={:?}", a, b);

    // ---------- 3. FFI: calling C code ----------
    // Requires `extern "C"` blocks.
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("abs(-3) = {}", abs(-3));
    }

    // Exposing a Rust fn to C:
    //   #[no_mangle]
    //   pub extern "C" fn call_from_c() {}

    // ---------- 4. MUTABLE STATIC VARIABLES ----------
    // Dangerous due to data races; prefer Mutex + OnceLock.
    static mut COUNTER: u32 = 0;
    unsafe {
        COUNTER += 1;
        println!("COUNTER = {}", COUNTER);
    }

    // ---------- 5. UNSAFE TRAITS ----------
    // You mark a trait `unsafe trait Foo` when implementing
    // it wrong can cause UB. Implementers must write `unsafe impl`.
    unsafe trait PromiseSendSync {}
    unsafe impl PromiseSendSync for u32 {}

    // ---------- UNIONS ----------
    // Like C unions - all fields share memory. Reading is unsafe.
    #[repr(C)]
    union IntOrFloat { i: u32, f: f32 }
    let u = IntOrFloat { i: 0x40490fdb };
    unsafe {
        println!("as float = {}", u.f);   // π approx
    }
}

// Classic example: two mutable slices of the same Vec.
// This is IMPOSSIBLE in safe Rust (two &mut overlap in borrow
// checker's view), but provably safe if we split at `mid`.
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// ---------- UB TO AVOID ----------
// Undefined Behavior in unsafe Rust includes:
//   - Dereferencing dangling/null/misaligned pointers
//   - Reading uninitialized memory
//   - Breaking aliasing rules (two &mut to same location)
//   - Data races
// The compiler assumes these never happen; if they do, anything goes.
