// ============================================================
// 17 - HASHMAPS
// ============================================================
// HashMap<K, V> stores key-value pairs using a hashing
// function. Like Vec, it stores data on the heap.
//
// Import: use std::collections::HashMap;
//
// Keys must implement the Eq + Hash traits. All keys must be
// the same type; all values must be the same type.
// ============================================================

use std::collections::HashMap;

fn main() {
    // ---------- CREATING ----------
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    // From iterators
    let teams = vec![String::from("Red"), String::from("Green")];
    let initial = vec![5, 15];
    let map: HashMap<_, _> = teams.into_iter().zip(initial.into_iter()).collect();
    println!("{:?}", map);

    // ---------- ACCESS ----------
    let key = String::from("Blue");
    // get returns Option<&V>
    match scores.get(&key) {
        Some(v) => println!("Blue: {}", v),
        None => println!("not found"),
    }

    // copied: Option<&V> -> Option<V> (when V: Copy)
    let score = scores.get(&key).copied().unwrap_or(0);
    println!("score = {}", score);

    // ---------- ITERATION ----------
    for (k, v) in &scores {
        println!("{} = {}", k, v);
    }

    // ---------- OWNERSHIP ----------
    // For owned types (String), the map takes ownership.
    let field = String::from("color");
    let value = String::from("blue");
    let mut props = HashMap::new();
    props.insert(field, value);
    // println!("{}", field);   // ERROR: moved

    // For Copy types (i32), they are copied.

    // ---------- UPDATING ----------
    // Overwrite
    scores.insert(String::from("Blue"), 25);

    // Only insert if absent (entry API)
    scores.entry(String::from("Purple")).or_insert(100);
    scores.entry(String::from("Blue")).or_insert(999);   // unchanged
    println!("{:?}", scores);

    // Update based on old value (classic word count pattern)
    let text = "hello world wonderful world";
    let mut counts: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let entry = counts.entry(word).or_insert(0);
        *entry += 1;
    }
    println!("{:?}", counts);

    // ---------- REMOVAL ----------
    scores.remove("Purple");
    println!("after remove: {:?}", scores);

    // ---------- CHECKING EXISTENCE ----------
    if scores.contains_key("Blue") {
        println!("Blue exists");
    }

    // ---------- LEN / IS_EMPTY / CLEAR ----------
    println!("size: {}", scores.len());
    scores.clear();
    println!("empty? {}", scores.is_empty());
}
