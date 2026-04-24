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


// ! without async runtime 
use rouille::{Response, router};

fn calculate_sum(n: i64) -> i64 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}
fn main() {
    println!("Server on http://127.0.0.1:8080");
    rouille::start_server("127.0.0.1:8080", move |request| {
        router!(request,
            (GET) (/) => {
                let sum = calculate_sum(1000_000_000);  
                Response::json(&serde_json::json!({
                    "sum": sum,
                }))
            },
            _ => Response::empty_404()
        )
    });
}


// ! with async runtime (tokio)  // wrong way to use async runtime, just for demonstration of async/await syntax. In real code, you would want to use a proper async server framework that manages the runtime for you (e.g. warp, axum, etc.) and not block the main thread with a long-running synchronous operation like calculate_sum.
use tokio::runtime::Runtime;
use rouille::{Response, router};
fn main() {
    println!("Server on http://127.0.0.1:8080");

    // Create a single runtime to use for all requests. In real code, you might want to use a more robust server framework that manages the runtime for you (e.g. warp, axum, etc.).
    let rt = Runtime::new().unwrap();

    rouille::start_server("127.0.0.1:8080", move |request| {
        router!(request,
            (GET) (/) => {
                let file_contents = rt.block_on(async {
                    tokio::fs::read_to_string("a.txt").await.unwrap()
                });
                Response::json(&serde_json::json!({
                    "file_contents": file_contents
                }))
            },
            _ => Response::empty_404()
        )   
    });
}


// ! right way

// There are 3 famous HTTP frameworks you can use that support async rust
// 1. Axum
// 2. Actix web
// 3. Poem

// ! with async runtime (actix-web)
use actix_web::{get, App, HttpServer, HttpResponse, Result};
use serde_json::json;

#[get("/")]
async fn read_file() -> Result<HttpResponse> {
    let file_contents = tokio::fs::read_to_string("a.txt")
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(HttpResponse::Ok().json(json!({
        "file_contents": file_contents
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server on http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .service(read_file)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}