// ============================================================
// 04 - COMPOUND TYPES (TUPLES & ARRAYS)
// ============================================================
// Compound types group multiple values into one type.
//
// TUPLES:
//   - Fixed length, can mix different types.
//   - Type: (T1, T2, T3, ...)
//   - Access by .0, .1, .2 or by destructuring.
//   - The empty tuple `()` is called the UNIT type.
//
// ARRAYS:
//   - Fixed length (known at compile time), all SAME type.
//   - Stored on the stack.
//   - Type: [T; N]   e.g. [i32; 5]
//   - Bounds checked at runtime; out-of-bounds index PANICS.
//
// For a growable, heap-allocated sequence use `Vec<T>` (file 15).
// For a borrowed view into part of an array/vec, use a slice
// `&[T]` (file 10).
// ============================================================

fn main() {
    // ---------- TUPLE BASICS ----------
    let tup: (i32, f64, char) = (500, 6.4, 'R');
    println!("tuple: ({}, {}, {})", tup.0, tup.1, tup.2);

    // destructuring
    let (x, y, z) = tup;
    println!("destructured: x={}, y={}, z={}", x, y, z);

    // tuples can hold tuples
    let nested = ((1, 2), (3, 4));
    println!("nested: {} {}", (nested.0).1, (nested.1).0);

    // unit type: a tuple with no values
    let unit: () = ();
    println!("unit value prints as nothing -> '{:?}'", unit);

    // returning multiple values from a function (idiomatic use of tuples)
    let (q, r) = divmod(17, 5);
    println!("17 / 5 = {} remainder {}", q, r);

    // ---------- ARRAY BASICS ----------
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("first: {}, len: {}", arr[0], arr.len());

    // initialize all elements to the same value
    let zeros = [0u8; 8];           // [0, 0, 0, 0, 0, 0, 0, 0]
    println!("zeros: {:?}", zeros);

    // arrays are Copy if their element type is Copy
    let copy_of_arr = arr;
    println!("original still valid: {:?}", arr);
    println!("copy: {:?}", copy_of_arr);

    // ---------- ITERATING AN ARRAY ----------
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri"];
    for d in days.iter() {
        print!("{} ", d);
    }
    println!();

    // with index
    for (i, d) in days.iter().enumerate() {
        println!("  {}: {}", i, d);
    }

    // ---------- BOUNDS CHECKING ----------
    let safe = arr.get(10);         // returns Option, no panic
    println!("arr.get(10) = {:?}", safe);
    // let bad = arr[10];           // PANIC at runtime

    // ---------- SLICES OF AN ARRAY ----------
    // A slice is a borrowed view into part of an array.
    let middle: &[i32] = &arr[1..4];
    println!("middle slice: {:?}", middle);

    // ---------- 2D ARRAY ----------
    let grid: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    println!("grid[1][2] = {}", grid[1][2]);
}

fn divmod(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}
