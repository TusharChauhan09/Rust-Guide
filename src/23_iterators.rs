// ============================================================
// 23 - ITERATORS
// ============================================================
// An iterator produces a sequence of values. In Rust they are
// LAZY: no work happens until you consume them.
//
// THE Iterator TRAIT:
//   trait Iterator {
//       type Item;
//       fn next(&mut self) -> Option<Self::Item>;
//       // many provided methods built on top of `next`
//   }
//
// THREE WAYS TO ITERATE A COLLECTION:
//   v.iter()        -> iterator of &T   (borrow)
//   v.iter_mut()    -> iterator of &mut T
//   v.into_iter()   -> iterator of T    (consumes collection)
//
// ADAPTERS (lazy, chain-able): map, filter, take, skip, zip,
//                              enumerate, chain, rev, ...
// CONSUMERS (drive iteration): collect, sum, count, for_each,
//                              fold, any, all, find, min, max
// ============================================================

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // ---------- BASIC USAGE ----------
    let mut it = v.iter();
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&2));

    // for desugars into iterator + .next() loop
    for x in &v {
        print!("{} ", x);
    }
    println!();

    // ---------- ADAPTERS ----------
    // Lazy: this line does NOTHING until consumed:
    let _unused = v.iter().map(|x| x + 1);

    // map + filter + collect
    let doubled_evens: Vec<i32> = v.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * 2)
        .collect();
    println!("doubled evens: {:?}", doubled_evens);

    // enumerate - yields (index, value)
    for (i, x) in v.iter().enumerate() {
        println!("{}: {}", i, x);
    }

    // zip two iterators together
    let names = vec!["alice", "bob", "carol"];
    let ages = vec![30, 25, 40];
    let paired: Vec<(&&str, &i32)> = names.iter().zip(ages.iter()).collect();
    println!("{:?}", paired);

    // take / skip
    let first3: Vec<_> = v.iter().take(3).collect();
    let after2: Vec<_> = v.iter().skip(2).collect();
    println!("take3={:?} skip2={:?}", first3, after2);

    // chain / rev
    let a = vec![1, 2]; let b = vec![3, 4];
    let joined: Vec<_> = a.iter().chain(b.iter()).collect();
    let reversed: Vec<_> = v.iter().rev().collect();
    println!("joined={:?} reversed={:?}", joined, reversed);

    // ---------- CONSUMERS ----------
    let sum: i32 = v.iter().sum();
    let count = v.iter().count();
    let product: i32 = v.iter().product();
    println!("sum={} count={} product={}", sum, count, product);

    let max = v.iter().max();
    let min = v.iter().min();
    println!("max={:?} min={:?}", max, min);

    let any_gt_3 = v.iter().any(|&x| x > 3);
    let all_positive = v.iter().all(|&x| x > 0);
    println!("any>3: {}, all>0: {}", any_gt_3, all_positive);

    let first_even = v.iter().find(|&&x| x % 2 == 0);
    println!("first even: {:?}", first_even);

    // fold = reduce with accumulator
    let total = v.iter().fold(0, |acc, &x| acc + x);
    println!("fold sum = {}", total);

    // for_each: iterate for side effects
    v.iter().for_each(|x| print!("[{}] ", x));
    println!();

    // ---------- CREATING ITERATORS FROM RANGES ----------
    let squares: Vec<i32> = (1..=5).map(|x| x * x).collect();
    println!("{:?}", squares);

    // ---------- CUSTOM ITERATOR ----------
    let c = Counter::new();
    let data: Vec<u32> = c.take(5).collect();
    println!("counter: {:?}", data);
}

// ---------- IMPLEMENTING Iterator ----------
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        self.count += 1;
        if self.count <= 5 {
            Some(self.count)
        } else {
            None
        }
    }
}
