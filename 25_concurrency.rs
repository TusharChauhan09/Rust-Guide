// ============================================================
// 25 - CONCURRENCY
// ============================================================
// Rust's ownership system guarantees thread safety at COMPILE
// TIME. This is called "fearless concurrency".
//
// TOOLS:
//   std::thread      - spawn OS threads
//   std::sync::mpsc  - channels for passing messages
//   Mutex<T>         - exclusive locked access to data
//   Arc<T>           - atomic reference-counted shared ownership
//
// MARKER TRAITS:
//   Send  - T can be moved to another thread
//   Sync  - &T can be shared between threads
// ============================================================

use std::thread;
use std::sync::{mpsc, Mutex, Arc};
use std::time::Duration;

fn main() {
    // ---------- SPAWNING A THREAD ----------
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("spawned thread: {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });
    for i in 1..=3 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(10));
    }
    handle.join().unwrap();    // wait for spawned thread

    // ---------- MOVING DATA INTO A THREAD ----------
    let v = vec![1, 2, 3];
    // `move` transfers ownership into the closure.
    let h = thread::spawn(move || {
        println!("vec = {:?}", v);
    });
    h.join().unwrap();

    // ---------- CHANNELS (message passing) ----------
    // mpsc = multiple producer, single consumer
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();       // multiple senders

    thread::spawn(move || {
        tx.send(String::from("hi from 1")).unwrap();
    });
    thread::spawn(move || {
        tx2.send(String::from("hi from 2")).unwrap();
    });

    // Iterating rx gets messages until all tx are dropped.
    for msg in rx.iter().take(2) {
        println!("got: {}", msg);
    }

    // ---------- MUTEX ----------
    let m = Mutex::new(5);
    {
        let mut guard = m.lock().unwrap();   // blocks until available
        *guard += 10;
    } // lock released here
    println!("m = {:?}", m);

    // ---------- SHARING WITH Arc<Mutex<T>> ----------
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let c = Arc::clone(&counter);
        let h = thread::spawn(move || {
            let mut n = c.lock().unwrap();
            *n += 1;
        });
        handles.push(h);
    }
    for h in handles { h.join().unwrap(); }
    println!("counter = {}", *counter.lock().unwrap());  // 10

    // ---------- Send + Sync ----------
    // Most types are automatically Send/Sync. Exceptions include
    // Rc<T> (not thread-safe) and raw pointers.
    //
    // Compile-time rejection prevents data races:
    //   let rc = Rc::new(5);
    //   thread::spawn(move || println!("{}", rc));  // ERROR
}
