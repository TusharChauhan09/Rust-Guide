// ============================================================
// 12 - ENUMS
// ============================================================
// Enums let you define a type by enumerating its possible
// variants. Each variant can hold different data.
//
// SYNTAX:
//   enum Name {
//       Variant1,
//       Variant2(T1, T2),          // tuple variant
//       Variant3 { x: T1, y: T2 }, // struct variant
//   }
//
// Rust has no `null`. Use Option<T> instead (covered later).
// ============================================================

// ---------- SIMPLE ENUM ----------
enum Direction {
    North,
    South,
    East,
    West,
}

// ---------- ENUM WITH DATA ----------
#[derive(Debug)]
enum Message {
    Quit,                        // no data
    Move { x: i32, y: i32 },     // named-field variant
    Write(String),               // tuple variant
    ChangeColor(i32, i32, i32),  // tuple variant
}

// ---------- METHODS ON ENUMS ----------
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("quit"),
            Message::Move { x, y } => println!("move to ({}, {})", x, y),
            Message::Write(text) => println!("write: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("color: {} {} {}", r, g, b)
            }
        }
    }
}

// ---------- RECURSIVE ENUM (requires Box) ----------
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // ---------- USING SIMPLE ENUM ----------
    let dir = Direction::North;
    move_in(dir);

    // ---------- USING DATA-HOLDING ENUM ----------
    let msgs = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 0, 0),
    ];
    for m in &msgs {
        m.call();
    }

    // ---------- OPTION<T>: built-in enum ----------
    // enum Option<T> { Some(T), None }
    let some_n: Option<i32> = Some(5);
    let no_n: Option<i32> = None;
    match some_n {
        Some(v) => println!("got {}", v),
        None => println!("nothing"),
    }
    let _ = no_n;

    // ---------- RECURSIVE LIST ----------
    use List::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    print_list(&list);
}

fn move_in(d: Direction) {
    match d {
        Direction::North => println!("going north"),
        Direction::South => println!("going south"),
        Direction::East => println!("going east"),
        Direction::West => println!("going west"),
    }
}

fn print_list(list: &List) {
    match list {
        List::Cons(v, rest) => {
            print!("{} -> ", v);
            print_list(rest);
        }
        List::Nil => println!("Nil"),
    }
}
