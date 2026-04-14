// ============================================================
// 07 - LOOPS (loop / while / for)
// ============================================================
// Rust has THREE looping constructs:
//
//   loop      - infinite loop, exit with `break`
//   while     - loop while a condition is true
//   for       - iterate over anything that implements Iterator
//
// All three are expressions. Specifically, `loop` can RETURN a
// value via `break value;`. `while` and `for` always evaluate
// to `()`.
//
// CONTROL FLOW INSIDE LOOPS:
//   break;            stop the loop
//   continue;         skip to the next iteration
//   break value;      (loop only) exit and produce a value
//
// LOOP LABELS:
//   'name: loop { ... break 'name; ... }
//   Useful for breaking out of an OUTER loop from inside a
//   nested loop. Labels start with a single quote.
//
// PREFER `for ... in` over `while i < n { ... i += 1 }`.
// The for/iterator form is faster (no bounds checks per iter)
// and harder to get wrong.
// ============================================================

fn main() {
    // ---------- loop (INFINITE) ----------
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;      // exit the loop AND yield a value
        }
    };
    println!("loop returned: {}", result);

    // ---------- while ----------
    let mut n = 5;
    while n > 0 {
        println!("countdown: {}", n);
        n -= 1;
    }
    println!("liftoff!");

    // ---------- for OVER A RANGE ----------
    // 1..5  -> 1,2,3,4         (exclusive end)
    // 1..=5 -> 1,2,3,4,5       (inclusive end)
    for i in 1..5 {
        print!("{} ", i);
    }
    println!();

    for i in (1..=5).rev() {        // reverse with .rev()
        print!("{} ", i);
    }
    println!();

    // ---------- for OVER A COLLECTION ----------
    let nums = [10, 20, 30, 40];
    for n in nums.iter() {
        println!("n = {}", n);
    }

    // with index, via enumerate()
    let words = ["alpha", "beta", "gamma"];
    for (i, w) in words.iter().enumerate() {
        println!("{}: {}", i, w);
    }

    // step_by for a stride
    for i in (0..20).step_by(5) {
        print!("{} ", i);           // 0 5 10 15
    }
    println!();

    // ---------- continue / break ----------
    for i in 0..10 {
        if i % 2 == 0 { continue; } // skip evens
        if i > 7      { break; }    // stop after 7
        print!("{} ", i);           // prints: 1 3 5 7
    }
    println!();

    // ---------- LOOP LABELS ----------
    'outer: for i in 0..5 {
        for j in 0..5 {
            if i + j == 5 {
                println!("breaking at i={}, j={}", i, j);
                break 'outer;       // breaks the OUTER loop
            }
        }
    }

    // ---------- RETURNING A VALUE FROM A LABELED loop ----------
    let found = 'search: loop {
        for x in 1..100 {
            if x * x == 49 {
                break 'search x;    // value 7 leaves the loop
            }
        }
        break 'search -1;
    };
    println!("found sqrt(49) = {}", found);

    // ---------- ITERATOR ADAPTERS (preview of file 23) ----------
    let total: i32 = (1..=10).sum();
    let evens: Vec<i32> = (1..=10).filter(|x| x % 2 == 0).collect();
    println!("sum 1..=10 = {}", total);
    println!("evens     = {:?}", evens);
}
