// ============================================================
// 15 - VECTORS (Vec<T>)
// ============================================================
// Vec<T> is a growable, heap-allocated array. All elements
// must have the same type T.
//
// CREATION:
//   let v: Vec<i32> = Vec::new();
//   let v = vec![1, 2, 3];          // vec! macro
//
// COMMON METHODS:
//   v.push(x)        add to end
//   v.pop()          remove from end, returns Option<T>
//   v.len()          number of elements
//   v.iter()         iterate immutably
//   v.iter_mut()     iterate mutably
//   v.get(i)         Option<&T> (safe)
//   v[i]             &T (panics if OOB)
// ============================================================

fn main() {
    // ---------- CREATION ----------
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);

    let v2 = vec![10, 20, 30];
    println!("{:?}", v2);

    // ---------- ACCESSING ELEMENTS ----------
    let third: &i32 = &v2[2];           // panics if OOB
    println!("third = {}", third);

    match v2.get(100) {                 // safe access
        Some(x) => println!("{}", x),
        None => println!("out of range"),
    }

    // ---------- ITERATING ----------
    for n in &v {
        print!("{} ", n);
    }
    println!();

    // mutate each element
    let mut v3 = vec![1, 2, 3];
    for n in &mut v3 {
        *n *= 10;    // dereference with *
    }
    println!("{:?}", v3);

    // ---------- STORING MULTIPLE TYPES via ENUM ----------
    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        Cell::Int(3),
        Cell::Float(10.12),
        Cell::Text(String::from("blue")),
    ];
    for c in &row {
        match c {
            Cell::Int(i) => println!("int: {}", i),
            Cell::Float(f) => println!("float: {}", f),
            Cell::Text(s) => println!("text: {}", s),
        }
    }

    // ---------- USEFUL METHODS ----------
    let mut nums = vec![3, 1, 4, 1, 5, 9, 2, 6];
    nums.sort();
    println!("sorted = {:?}", nums);

    nums.reverse();
    println!("reversed = {:?}", nums);

    let total: i32 = nums.iter().sum();
    let max = nums.iter().max();
    println!("sum = {}, max = {:?}", total, max);

    // contains / remove
    println!("has 4? {}", nums.contains(&4));
    nums.retain(|&x| x > 2);    // keep only > 2
    println!("retained = {:?}", nums);

    // with_capacity for performance
    let mut big: Vec<i32> = Vec::with_capacity(1000);
    for i in 0..5 { big.push(i); }
    println!("len={}, cap={}", big.len(), big.capacity());

    // ---------- SLICING A VEC ----------
    let v4 = vec![1, 2, 3, 4, 5];
    let slice: &[i32] = &v4[1..4];
    println!("slice = {:?}", slice);
}
