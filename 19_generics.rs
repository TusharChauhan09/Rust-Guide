// ============================================================
// 19 - GENERICS
// ============================================================
// Generics let you write code that works over many types
// without runtime cost (monomorphized at compile time).
//
// SYNTAX:
//   fn name<T>(x: T) -> T { ... }
//   struct Foo<T> { v: T }
//   impl<T> Foo<T> { ... }
//
// TRAIT BOUNDS tell the compiler what the generic type must
// support:
//   fn largest<T: PartialOrd>(list: &[T]) -> &T { ... }
//   fn show<T: std::fmt::Display>(x: T) { ... }
// ============================================================

use std::fmt::Display;
use std::ops::Add;

// ---------- GENERIC FUNCTION ----------
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// ---------- MULTIPLE GENERIC PARAMS ----------
fn pair<A, B>(a: A, b: B) -> (A, B) {
    (a, b)
}

// ---------- GENERIC STRUCT ----------
struct Point<T> {
    x: T,
    y: T,
}

// Impl block for any T
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    fn x(&self) -> &T {
        &self.x
    }
}

// Impl block ONLY for Point<f64> (specialization-like)
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// Generic struct with two type params
struct Pair<A, B> {
    first: A,
    second: B,
}

// ---------- GENERIC ENUM ----------
// Option<T> and Result<T, E> are generic enums.
enum MyOption<T> {
    Some(T),
    None,
}

// ---------- BOUNDS WITH where CLAUSE (cleaner) ----------
fn announce<T, U>(t: T, u: U) -> String
where
    T: Display + Clone,
    U: Display,
{
    format!("{} with {}", t.clone(), u)
}

// ---------- MULTIPLE BOUNDS ----------
fn sum<T: Add<Output = T> + Copy>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let nums = vec![34, 50, 25, 100, 65];
    println!("largest = {}", largest(&nums));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("largest = {}", largest(&chars));

    let p = Point::new(5, 10);
    println!("x = {}", p.x());

    let f = Point::new(3.0, 4.0);
    println!("dist = {}", f.distance_from_origin());

    let _mixed: Pair<i32, &str> = Pair { first: 1, second: "two" };

    println!("{}", announce("hello", 42));
    println!("{}", sum(3, 4));
    println!("{}", sum(1.5, 2.5));

    let (a, b) = pair("hi", 42);
    println!("{} {}", a, b);

    // MyOption
    let x: MyOption<i32> = MyOption::Some(5);
    match x {
        MyOption::Some(v) => println!("got {}", v),
        MyOption::None => println!("none"),
    }
}

// NOTE: generics are MONOMORPHIZED. Compiler creates concrete
// copies per type used. Zero runtime cost vs hand-written code.
