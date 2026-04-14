// ============================================================
// 30 - TESTING
// ============================================================
// Rust has first-class built-in testing. Run with `cargo test`.
//
// KINDS OF TESTS:
//   Unit tests        -> in the same file, in a #[cfg(test)] module
//   Integration tests -> in tests/*.rs (separate crate)
//   Doc tests         -> inside doc comments (///)
//
// ATTRIBUTES:
//   #[test]           mark a function as a test
//   #[should_panic]   test must panic
//   #[ignore]         skip unless --ignored is passed
//   #[cfg(test)]      compile only when testing
//
// ASSERTION MACROS:
//   assert!(cond)
//   assert_eq!(a, b)
//   assert_ne!(a, b)
// ============================================================

// ---------- FUNCTIONS BEING TESTED ----------
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 { panic!("divide by zero"); }
    a / b
}

/// Returns the greeting for a name.
///
/// # Examples
///
/// ```
/// // This block is a DOC TEST: cargo test runs it.
/// // (in a real crate the assertion below would be live)
/// // assert_eq!(my_crate::greet("Rust"), "Hello, Rust!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    // Just a demo so this file can be a binary.
    println!("{}", greet("world"));
    println!("{} / {} = {}", 10, 2, divide(10, 2));
}

// ---------- UNIT TEST MODULE ----------
// `#[cfg(test)]` means this module is only compiled for tests.
#[cfg(test)]
mod tests {
    use super::*;                 // bring parent items into scope

    #[test]
    fn adds_positive() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn adds_negative() {
        assert_eq!(add(-1, -1), -2);
    }

    #[test]
    fn greets() {
        let s = greet("Rust");
        assert!(s.contains("Rust"));
        assert_eq!(s, "Hello, Rust!");
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn panics_on_zero() {
        divide(1, 0);
    }

    #[test]
    #[ignore]
    fn slow_test() {
        // run only when: cargo test -- --ignored
    }

    // ---------- TESTS THAT RETURN Result ----------
    #[test]
    fn it_works() -> Result<(), String> {
        if add(2, 2) == 4 { Ok(()) } else { Err(String::from("math broke")) }
    }
}

// ============================================================
// RUNNING TESTS
// ============================================================
//   cargo test                  # run all tests
//   cargo test tests::adds      # run tests whose name matches
//   cargo test -- --nocapture   # show println! output
//   cargo test -- --test-threads=1   # run sequentially
//   cargo test -- --ignored     # run only #[ignore] tests
//
// ============================================================
// INTEGRATION TESTS
// ============================================================
// Place them in a top-level `tests/` directory, e.g.:
//
//   tests/integration.rs
//
//     use my_crate::add;
//     #[test]
//     fn integration_add() {
//         assert_eq!(add(2, 2), 4);
//     }
//
// Each file in `tests/` is compiled as a SEPARATE crate.
// ============================================================
