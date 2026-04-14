// ============================================================
// 24 - SMART POINTERS
// ============================================================
// Smart pointers are types that act like a pointer but also
// have additional metadata and capabilities. They usually
// own the data they point to.
//
// Key smart pointers in the std library:
//   Box<T>       - heap allocation, single owner
//   Rc<T>        - reference-counted, multiple owners (single-thread)
//   Arc<T>       - atomic Rc (thread-safe)
//   RefCell<T>   - interior mutability, runtime borrow checks
//   Mutex<T>     - thread-safe interior mutability
//
// Implement key traits:
//   Deref  - overloads *
//   Drop   - custom cleanup when dropped
// ============================================================

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

fn main() {
    // ========== Box<T> ==========
    // Allocates T on the heap. Use for:
    //   - large data you want to move cheaply
    //   - recursive types with unknown compile-time size
    //   - trait objects (Box<dyn Trait>)
    let b = Box::new(5);
    println!("b = {}", b);

    // Recursive type (cons list)
    let list = ConsList::Cons(1, Box::new(ConsList::Cons(2, Box::new(ConsList::Nil))));
    print_cons(&list);

    // ========== Deref ==========
    let m = MyBox::new(5);
    println!("*m = {}", *m);        // calls .deref()

    // Deref coercion: &MyBox<String> -> &String -> &str
    let name = MyBox::new(String::from("Rust"));
    hello(&name);

    // ========== Drop ==========
    let _cws = CustomSmart { data: String::from("A") };
    let _cws2 = CustomSmart { data: String::from("B") };
    println!("about to exit main...");
    // Drops happen in reverse order of creation (B, then A).

    // ========== Rc<T> ==========
    // Multiple ownership via reference counting. Immutable.
    let a = Rc::new(vec![1, 2, 3]);
    println!("count after a: {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);      // cheap, just increments count
    println!("count after b: {}", Rc::strong_count(&a));
    {
        let _c = Rc::clone(&a);
        println!("count in scope: {}", Rc::strong_count(&a));
    }
    println!("count after scope: {}", Rc::strong_count(&a));
    println!("b = {:?}", b);

    // ========== RefCell<T> ==========
    // Interior mutability: mutate via shared ref (&T).
    // Borrow rules checked at RUNTIME (panic if violated).
    let cell = RefCell::new(5);
    *cell.borrow_mut() += 10;
    println!("cell = {}", cell.borrow());

    // ========== Rc<RefCell<T>> ==========
    // Common combo: shared ownership + mutation.
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
    let s1 = Rc::clone(&shared);
    let s2 = Rc::clone(&shared);
    s1.borrow_mut().push(4);
    s2.borrow_mut().push(5);
    println!("shared = {:?}", shared.borrow());
}

// ---------- Recursive enum using Box ----------
enum ConsList {
    Cons(i32, Box<ConsList>),
    Nil,
}

fn print_cons(list: &ConsList) {
    match list {
        ConsList::Cons(v, next) => {
            print!("{} -> ", v);
            print_cons(next);
        }
        ConsList::Nil => println!("Nil"),
    }
}

// ---------- Custom smart pointer implementing Deref ----------
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> { MyBox(x) }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// ---------- Drop trait ----------
struct CustomSmart { data: String }

impl Drop for CustomSmart {
    fn drop(&mut self) {
        println!("Dropping CustomSmart with data `{}`", self.data);
    }
}
