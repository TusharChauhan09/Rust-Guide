# Rust Topics

A curated, ordered set of single-file Rust examples covering the language from the basics through the advanced topics. Each file is self-contained: read the comments top-to-bottom and run it with `rustc`.

## How to use

Each `.rs` file is a standalone program. Run any one with:

```bash
rustc src/01_hello_world.rs && ./01_hello_world
```

Or, if you prefer Cargo, drop a file into a `src/main.rs` of a new project.

Read the comments first — they explain the concept, syntax, and gotchas. The `main` function demonstrates the concept in runnable code.

## Topics (in learning order)

| #   | File                                                                 | Topic                                                |
| --- | -------------------------------------------------------------------- | ---------------------------------------------------- |
| 01  | [01_hello_world.rs](src/01_hello_world.rs)                           | `println!`, format placeholders, comments, `stdin`   |
| 02  | [02_variables_and_mutability.rs](src/02_variables_and_mutability.rs) | `let`, `mut`, `const`, shadowing                     |
| 03  | [03_data_types.rs](src/03_data_types.rs)                             | Integers, floats, `bool`, `char`, casting            |
| 04  | [04_compound_types.rs](src/04_compound_types.rs)                     | Tuples and arrays                                    |
| 05  | [05_functions.rs](src/05_functions.rs)                               | `fn`, parameters, returns, expressions vs statements |
| 06  | [06_control_flow.rs](src/06_control_flow.rs)                         | `if` / `else if` / `else` as expressions             |
| 07  | [07_loops.rs](src/07_loops.rs)                                       | `loop`, `while`, `for`, labels, `break value`        |
| 08  | [08_ownership.rs](src/08_ownership.rs)                               | Ownership, move vs copy, scope                       |
| 09  | [09_borrowing_and_references.rs](src/09_borrowing_and_references.rs) | `&T`, `&mut T`, borrow rules                         |
| 10  | [10_slices.rs](src/10_slices.rs)                                     | `&str`, `&[T]`, range syntax                         |
| 11  | [11_structs.rs](src/11_structs.rs)                                   | Named / tuple / unit structs, `impl`                 |
| 12  | [12_enums.rs](src/12_enums.rs)                                       | Variants with data, recursive enums                  |
| 13  | [13_pattern_matching.rs](src/13_pattern_matching.rs)                 | `match`, `if let`, `while let`, guards               |
| 14  | [14_option_and_result.rs](src/14_option_and_result.rs)               | `Option<T>`, `Result<T, E>`, `?`                     |
| 15  | [15_vectors.rs](src/15_vectors.rs)                                   | `Vec<T>` and common methods                          |
| 16  | [16_strings.rs](src/16_strings.rs)                                   | `String` vs `&str`, UTF-8                            |
| 17  | [17_hashmaps.rs](src/17_hashmaps.rs)                                 | `HashMap<K, V>`, the entry API                       |
| 18  | [18_error_handling.rs](src/18_error_handling.rs)                     | `panic!`, `Result`, error propagation                |
| 19  | [19_generics.rs](src/19_generics.rs)                                 | Generic functions, structs, enums                    |
| 20  | [20_traits.rs](src/20_traits.rs)                                     | Traits, default methods, trait bounds                |
| 21  | [21_lifetimes.rs](src/21_lifetimes.rs)                               | `'a`, elision, `'static`                             |
| 22  | [22_closures.rs](src/22_closures.rs)                                 | `Fn`, `FnMut`, `FnOnce`, `move`                      |
| 23  | [23_iterators.rs](src/23_iterators.rs)                               | Iterator adapters & consumers                        |
| 24  | [24_smart_pointers.rs](src/24_smart_pointers.rs)                     | `Box`, `Rc`, `RefCell`, `Deref`, `Drop`              |
| 25  | [25_concurrency.rs](src/25_concurrency.rs)                           | Threads, channels, `Mutex`, `Arc`                    |
| 26  | [26_async_await.rs](src/26_async_await.rs)                           | `async fn`, `.await`, futures                        |
| 27  | [27_macros.rs](src/27_macros.rs)                                     | `macro_rules!`, proc-macros overview                 |
| 28  | [28_unsafe_rust.rs](src/28_unsafe_rust.rs)                           | Raw pointers, FFI, unsafe traits                     |
| 29  | [29_modules_and_crates.rs](src/29_modules_and_crates.rs)             | `mod`, `pub`, paths, packages                        |
| 30  | [30_testing.rs](src/30_testing.rs)                                   | `#[test]`, unit / integration / doc tests            |

## Suggested path

1. **Basics (01–07):** syntax, types, functions, control flow — the fundamentals shared with most languages.
2. **Foundations (08–14):** memory model and pattern matching. These are the ideas that make Rust feel different.
3. **Collections (15–18):** `Vec`, `String`, `HashMap`, and how errors flow.
4. **Abstractions (19–23):** generics, traits, lifetimes, closures, iterators — the tools for writing reusable code.
5. **Systems (24–28):** smart pointers, concurrency, async, macros, unsafe — the advanced knobs.
6. **Project skills (29–30):** modules and testing, for structuring real crates.

## Conventions used in the files

- `// ---------- SECTION ----------` divides each file into labelled sections.
- Comments explain **why** and **syntax**, not just what the code does.
- `// ERROR: ...` marks lines that are commented out because they would fail to compile, with a note on why.

## References

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) (for unsafe Rust)
- [Standard library docs](https://doc.rust-lang.org/std/)
