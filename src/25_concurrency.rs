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
    handle.join().unwrap();    // join : wait for spawned thread tofinish and then exist the block of code OR move to next line of code
                               // unwrap : if the thread panics, join() returns an Err, otherwise Ok(()). unwrap() will panic if it receives an Err, so this will cause the main thread to panic if the spawned thread panics. If the spawned thread finishes successfully, join() returns Ok(()), and unwrap() does nothing.   

    // ---------- MOVING DATA INTO A THREAD ----------

    // why move? Because the spawned thread may outlive the current scope, so we need to transfer ownership of any data it uses. The `move` keyword forces the closure to take ownership of the variables it uses from the surrounding scope. This ensures that the data is safely transferred to the new thread and prevents any potential data races or dangling references. In this example, we move the vector `v` into the closure, allowing the spawned thread to access it without any issues. If we didn't use `move`, the closure would try to borrow `v`, which could lead to a compile-time error since the main thread might still be using `v` when the spawned thread tries to access it.
    let v = vec![1, 2, 3];
    // `move` transfers ownership into the closure.
    let h = thread::spawn(move || {
        println!("vec = {:?}", v);
    });
    h.join().unwrap();

    //! ---------- THREAD SAFETY WITH OWNERSHIP ----------
    // in order to avoid race conditions, Rust enforces that only one thread can have mutable access to data at a time. This is achieved through the ownership system, which ensures that data is either owned by a single thread or shared immutably across threads. If you want to share mutable data between threads, you can use synchronization primitives like Mutex or channels to ensure that only one thread can access the data at a time. This way, Rust guarantees thread safety at compile time and prevents data.

    //! A ----------Shared-State Concurrency
    use std::sync::Mutex;
    fn main() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {m:?}");
    }

    //! B ---------- CHANNELS (message passing) ----------
    // message passing is a common way to communicate between threads. A channel has a sender and a receiver. The sender can be cloned to allow multiple producers, but there is only one receiver. The receiver can iterate over incoming messages until all senders are dropped. This allows for safe communication between threads without the need for locks, as the ownership of the messages is transferred through the channel. In this example, we create a channel and spawn two threads that send messages to the receiver. The main thread then iterates over the received messages and prints them out.
    // mpsc = multiple producer, single consumer
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();       // multiple senders

    thread::spawn(move || {
        // send
        tx.send(String::from("hi from 1")).unwrap();
    });
    thread::spawn(move || {
        tx2.send(String::from("hi from 2")).unwrap();
    });

        // recv

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


// calculate sum for a large number
fn calculate_sum(){
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    // 2 threads to calculate sum : 0-1000 
    thread::spawn(move || {
        let sum: u32 = (1..=500).sum();   // short for sum the n numbers
        tx.send(sum).unwrap();
    });

    thread::spawn(move || {
        let sum: u32 = (501..=1000).sum();
        tx2.send(sum).unwrap();
    });

    let mut sum: u32 = 0;
    for msg in rx.iter().take(2) {
        sum += msg;
    }
    println!("Sum: {}", sum);
}   

// OR 
fn calculate_optimal(){
    let (tx, rx) = mpsc::channel();
    for i in 0..10 {   // 10 threads to calculate sum : 0-1000  // i.e 10 times the sum 0..100 
        let producer = tx.clone();
        thread::spawn(move || {
            let mut sum =0 ;
            for j in (i*100+1)..=((i+1)*100) {
                sum += j;
            }
            producer.send(sum).unwrap();
        });
    }
    drop(tx);
    
    let mut sum = 0;
    for val in rx{
        sum += val
    }
    println!("Sum: {}", sum);
}



// mpsc with tokio making it async mpsc channel
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);

    tokio::spawn(async move {
        tx.send("hi from task").await.unwrap();
    });

    // Doesn't block thread, just awaits asynchronously
    if let Some(msg) = rx.recv().await {
        println!("Got: {}", msg);
    }
}