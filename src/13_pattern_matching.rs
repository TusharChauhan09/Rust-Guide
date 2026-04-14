// ============================================================
// 13 - PATTERN MATCHING (match, if let, while let)
// ============================================================
// Pattern matching is exhaustive: the compiler ensures you
// cover every possible variant/pattern.
//
// PATTERN KINDS:
//   - Literal:    1, 'c', "hi"
//   - Range:      1..=5
//   - Variable:   x, name
//   - Wildcard:   _
//   - Struct/enum destructuring
//   - OR:         A | B
//   - Guards:     x if x > 5
//   - Bindings:   name @ 1..=5
// ============================================================

fn main() {
    // ---------- BASIC match ----------
    let n = 3;
    match n {
        1 => println!("one"),
        2 | 3 => println!("two or three"),
        4..=9 => println!("between 4 and 9"),
        _ => println!("other"),        // _ = wildcard, catch-all
    }

    // ---------- match with return value ----------
    let word = match n {
        0 => "zero",
        1 => "one",
        _ => "many",
    };
    println!("word = {}", word);

    // ---------- MATCH ON ENUM ----------
    let coin = Coin::Quarter(State::Alaska);
    let cents = value_in_cents(coin);
    println!("{} cents", cents);

    // ---------- MATCH ON Option ----------
    let x: Option<i32> = Some(5);
    let plus_one = match x {
        Some(i) => Some(i + 1),
        None => None,
    };
    println!("{:?}", plus_one);

    // ---------- DESTRUCTURING STRUCTS ----------
    let p = Point { x: 3, y: 7 };
    let Point { x, y } = p;      // destructure in let
    println!("x={} y={}", x, y);

    match p {
        Point { x: 0, y } => println!("on y-axis at {}", y),
        Point { x, y: 0 } => println!("on x-axis at {}", x),
        Point { x, y } => println!("at ({}, {})", x, y),
    }

    // ---------- DESTRUCTURING TUPLES ----------
    let pair = (1, -1);
    match pair {
        (0, _) => println!("first is 0"),
        (_, 0) => println!("second is 0"),
        (a, b) if a == -b => println!("opposite"),
        _ => println!("other"),
    }

    // ---------- IF LET (concise single-pattern) ----------
    let some_val = Some(7);
    if let Some(v) = some_val {
        println!("if let got {}", v);
    } else {
        println!("nothing");
    }

    // ---------- WHILE LET ----------
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("popped {}", top);
    }

    // ---------- @ BINDINGS ----------
    let msg = 5;
    match msg {
        n @ 1..=5 => println!("small: {}", n),
        n @ 6..=10 => println!("medium: {}", n),
        _ => println!("other"),
    }

    // ---------- let-else (stable) ----------
    let s = "42";
    let Ok(num) = s.parse::<i32>() else {
        println!("not a number");
        return;
    };
    println!("parsed: {}", num);
}

enum State {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn value_in_cents(c: Coin) -> u8 {
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            match state {
                State::Alabama => println!("from Alabama"),
                State::Alaska => println!("from Alaska"),
            }
            25
        }
    }
}

struct Point { x: i32, y: i32 }
