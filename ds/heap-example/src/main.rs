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

    println!("ancestors of element at index 5, including itself:");
    for ancestor in ancestors(&h, 5) {
        print!("({ancestor}) ");
    }
    println!();
}

fn ancestors<T, K>(heap: &Heap<T, K>, index: usize) -> impl Iterator<Item = &T> {
    let mut maybe_current = Some(index);
    std::iter::from_fn(move || {
        let c = maybe_current?;
        let el = heap.get(c)?;
        maybe_current = heap.parent_index(c).and_then(Result::ok);
        Some(el)
    })
}
