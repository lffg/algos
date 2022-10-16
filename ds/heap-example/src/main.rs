use std::fmt::Debug;

use heap::{printer::HeapPrinter, Heap};

fn p<T: Debug>(heap: &Heap<T>) {
    print!("{}", HeapPrinter(heap));
    println!("------------------------");
}

fn main() {
    let mut h = Heap::<i32>::new();
    p(&h);
    for &i in &[5, 3, 4, 2, 1, 6] {
        println!("inserted {i}, resulted in:");
        h.insert(i);
        p(&h);
    }
}
