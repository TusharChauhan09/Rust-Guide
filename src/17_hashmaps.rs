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

    // ! creation 

    // a Empty HashMap
    // Method 1: new()
    let mut scores: HashMap<String, i32> = HashMap::new();


    // b With Initial Values
    // Using insert()
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 25);

    // Using macro (not built-in, but can create)
    let map = HashMap::from([
        ("key1", 1),
        ("key2", 2),
        ("key3", 3),
    ]);

    // ! Operations

    // a Insert and Update

    scores.insert("Blue".to_string(), 10);
    scores.insert("Red".to_string(), 25);

    // Update existing value
    scores.insert("Blue".to_string(), 15);  // Overwrites

    // Insert only if key doesn't exist
    scores.entry("Yellow".to_string()).or_insert(30);
    scores.entry("Blue".to_string()).or_insert(99);  // Won't change Blue (still 15)

    // Insert with custom logic
    scores.entry("Green".to_string())
        .and_modify(|v| *v += 5)
        .or_insert(20);

    // b. get values 
    // Get by reference (returns Option<&V>)
    let blue_score = scores.get("Blue");
    println!("{:?}", blue_score);  // Some(10)

    // Get with default
    let yellow_score = scores.get("Yellow").unwrap_or(&0);
    println!("{}", yellow_score);  // 0

    // Get mutable reference
    if let Some(score) = scores.get_mut("Blue") {
        *score += 5;
    }
    println!("{:?}", scores.get("Blue"));  // Some(15)

    // c remove value
    // Remove by key (returns Option<V>)
    let removed = scores.remove("Blue");
    println!("{:?}", removed);  // Some(10)

    // Remove if condition met
    scores.remove_entry("Red");  // Removes and returns (key, value)

    // Clear all
    scores.clear();


    // ! iterate
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Red".to_string(), 25);
    scores.insert("Green".to_string(), 15);

    // Iterate over key-value pairs
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Iterate over keys
    for key in scores.keys() {
        println!("Key: {}", key);
    }

    // Iterate over values
    for value in scores.values() {
        println!("Value: {}", value);
    }

    // Mutable iteration
    for (_, value) in scores.iter_mut() {
        *value += 10;
    }


    // !  Common HashMap Methods

    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    // Check existence
    if map.contains_key("a") {
        println!("Key 'a' exists");
    }

    // Get length
    println!("Size: {}", map.len());  // 3
    println!("Is empty: {}", map.is_empty());  // false

    // Entry API for complex operations
    map.entry("d").or_insert(4);  // Insert if not exists

    map.entry("a")
        .and_modify(|v| *v += 10)
        .or_insert(100);



    // !  freq code
    let arr = vec![1, 2, 2, 3, 1, 4, 2];
    let mut freq = HashMap::new();
    for num in arr {
        *freq.entry(num).or_insert(0) += 1;
    }
    println!("{:?}", freq);
}


