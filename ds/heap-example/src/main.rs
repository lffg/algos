use std::fmt::Debug;

use heap::{printer::HeapPrinter, Heap, Max};

fn p<T, K>(heap: &Heap<T, K>)
where
    T: Debug,
{
    print!("{}", HeapPrinter(heap));
    println!("------------------------");
}

fn main() {
    let mut h = Heap::<i32, Max>::new();
    p(&h);
    for &i in &[5, 3, 4, 2, 1, 6] {
        println!("inserted {i}, resulted in:");
        h.insert(i);
        p(&h);
    }
}
