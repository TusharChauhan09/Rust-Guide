// ============================================================
// 26 - ASYNC / AWAIT
// ============================================================
// Async Rust enables concurrent I/O without spawning threads.
// It compiles async fns into state machines driven by an
// executor (tokio, async-std, smol, etc.).
//
// KEY CONCEPTS:
//   async fn foo() -> T      returns a Future<Output = T>
//   .await                   suspends until the Future resolves
//   Future                   a value that may not be ready yet
//   Executor / Runtime       drives futures to completion
//
// NOTE: The standard library provides the Future trait, but
// does NOT provide a runtime. You need a crate like `tokio`.
//
// DEPENDENCY (Cargo.toml):
//   tokio = { version = "1", features = ["full"] }
// ============================================================

// ---------- BASIC ASYNC FUNCTION ----------
async fn hello() -> String {
    String::from("hello")
}

async fn add(a: i32, b: i32) -> i32 {
    a + b
}

// ---------- AWAITING OTHER ASYNC FNS ----------
async fn greet(name: &str) -> String {
    let greeting = hello().await;           // await a Future
    let total = add(1, 2).await;
    format!("{}, {}! ({})", greeting, name, total)
}

// ---------- CONCURRENCY WITH join! ----------
// tokio::join! runs multiple futures concurrently on one task.
// async fn multi() -> (String, i32) {
//     let (g, n) = tokio::join!(hello(), add(10, 20));
//     (g, n)
// }

// ---------- SPAWNING A TASK ----------
// tokio::spawn moves a future onto the runtime so it runs
// independently. Returns a JoinHandle<T>.
// #[tokio::main]
// async fn main() {
//     let h = tokio::spawn(async {
//         for i in 0..3 {
//             println!("tick {}", i);
//             tokio::time::sleep(Duration::from_millis(50)).await;
//         }
//     });
//     let s = greet("tushar").await;
//     println!("{}", s);
//     h.await.unwrap();
// }

// ---------- SIMPLE SYNC main (no runtime) ----------
// This file compiles without tokio by using block_on-like logic
// for demonstration only. Real code uses a runtime.
fn main() {
    // Building a future without running it prints nothing:
    let _f = hello();
    // To actually run futures you need an executor, e.g.:
    //
    //   let rt = tokio::runtime::Runtime::new().unwrap();
    //   rt.block_on(async {
    //       println!("{}", greet("tushar").await);
    //   });

    println!("(see comments for tokio runtime usage)");
}

// ---------- WHY ASYNC vs THREADS ----------
// Threads:  OS-managed, heavy (~1 MB stack), great for CPU work.
// Async:    very cheap (~bytes per task), great for I/O-bound
//           code (servers, network clients, timers).
//
// ---------- PITFALLS ----------
// - Futures do nothing unless awaited or spawned.
// - Holding a MutexGuard across an .await can deadlock.
// - Use tokio::sync::Mutex, not std::sync::Mutex, across .awaits.
// - !Send futures cannot be spawned across threads.
