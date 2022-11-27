use std::fmt::{Debug, Display, Formatter, Result, Write};

use crate::Heap;

pub const INDENT_SIZE: usize = 4;

pub struct HeapPrinter<'a, T>(pub &'a Heap<T>);

impl<'a, T> Display for HeapPrinter<'a, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let HeapPrinter(heap) = *self;
        if heap.is_empty() {
            writeln!(f, "(empty heap)")
        } else {
            go(f, heap, 0)
        }
    }
}

fn go<T>(f: &mut Formatter<'_>, heap: &Heap<T>, index: usize) -> Result
where
    T: Debug,
{
    let Some(value) = heap.get(index) else {
        return Ok(());
    };

    let depth_0 = Heap::<T>::depth_at(index) - 1;
    repeat(f, ' ', depth_0 * INDENT_SIZE)?;
    writeln!(f, "{value:?}")?;

    let (a, b) = Heap::<T>::children_indexes(index);
    go(f, heap, a)?;
    go(f, heap, b)?;

    Ok(())
}

fn repeat(f: &mut Formatter<'_>, c: char, count: usize) -> Result {
    for _ in 0..count {
        f.write_char(c)?;
    }
    Ok(())
}
