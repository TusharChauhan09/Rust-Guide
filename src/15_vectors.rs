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
    // ! or let mut v: Vec<i32> = vec![]; type is anotated as Vec<i32> 
    // ! or let mut v = vec![0;0]; type is infered as [0;0] -> Vec<i32>
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

    // iteration 
    let vec = vec![1, 2, 3, 4, 5];

    // a for in 
    // Using iter()
    for item in vec.iter() {
        println!("{}", item);  // item is &i32
    }
    // Same as above (automatic deref coercion)
    for item in &vec {
        println!("{}", item);
    }

    // b. indices
    // Using range
    for i in 0..vec.len() {
        println!("vec[{}] = {}", i, vec[i]);
    }

    // c. enumerate
    for (index, value) in vec.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }

    
    // ! 2d vectors
    // Using Vec::new()
    let mut grid: Vec<Vec<i32>> = Vec::new();

    // Using vec![] macro
    let mut grid: Vec<Vec<i32>> = vec![];
    // with some size
    let mut grid: Vec<Vec<i32>> = vec![vec![0; 3]; 3]; // 3x3 all 0 values
    
    // Add rows later
    grid.push(vec![1, 2, 3]);
    grid.push(vec![4, 5, 6]);


    // 2x3 matrix
    let grid = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
    ];


    // itterate 2d vector
    let matrix = vec![
    vec![1, 2, 3],
    vec![4, 5, 6],
    vec![7, 8, 9],
    ];

    // a. iter()
    // Using iter() explicitly
    for row in matrix.iter() {
        for item in row.iter() {
            print!("{} ", item);
        }
        println!();
    }
    // Output:
    // 1 2 3 
    // 4 5 6 
    // 7 8 9

    // Same as above (automatic deref coercion)
    for row in &matrix {
        for item in row {
            print!("{} ", item);
        }
        println!();
    }

    // b. indices
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            println!("matrix[{}][{}] = {}", i, j, matrix[i][j]);
        }
    }

    // c. enumerate
    for (i, row) in matrix.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            println!("Index: ({}, {}), Value: {}", i, j, value);
        }
    }

}
