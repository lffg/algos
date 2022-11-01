pub mod printer;

/// Maximum heap implementation.
///
/// Use `std::order::Reverse` if you want a minimum heap behavior.
pub struct Heap<T> {
    inner: Vec<T>,
}

impl<T> Heap<T> {
    /// Constructs a new, empty, heap.
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Returns the length of the heap.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Checks whether the heap is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns the element at the given position.
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index)
    }
}

impl<T: Ord> Heap<T> {
    /// Inserts the given value on the heap.
    pub fn insert(&mut self, value: T) {
        let mut c = self.put(value);
        while c > 0 {
            let parent = Self::parent(c);
            if Self::is_higher(&self.inner[c], &self.inner[parent]) {
                self.inner.swap(c, parent);
                c = parent;
            } else {
                break;
            }
        }
    }

    /// Removes the maximum element.
    pub fn remove(&mut self) -> Option<T> {
        Self::logical_remove(&mut self.inner).then(|| self.inner.pop().unwrap())
    }

    /// Consumes the heap, returning an ordered vector.
    pub fn sorted(self) -> Vec<T> {
        let mut xs = self.inner;
        let mut len = xs.len();
        while len > 1 {
            Self::logical_remove(&mut xs[..len]);
            len -= 1;
        }
        xs
    }
}

// Utilities
impl<T> Heap<T> {
    /// Inserts a new element at the end of the internal buffer and returns its
    /// index.
    #[inline]
    fn put(&mut self, value: T) -> usize {
        self.inner.push(value);
        self.inner.len() - 1
    }

    /// Checks whether the element on the given `index` is a leaf in a heap with
    /// the provided length.
    #[inline]
    fn is_leaf(len: usize, index: usize) -> bool {
        index >= len / 2
    }

    /// Checks whether the element on the given `index` has any children in a
    /// heap with the provided length.
    #[inline]
    fn has_any_children(len: usize, index: usize) -> bool {
        !Self::is_leaf(len, index)
    }

    /// Returns the theoretical depth at the given index.
    #[inline]
    pub fn depth_at(index: usize) -> usize {
        let pos = index + 1;
        (pos as f64).log2().floor() as usize + 1
    }

    /// Returns the parent index of the node at the given index.
    ///
    /// # Panics
    ///
    /// This may overflow if `0` is given.
    #[inline]
    fn parent(child: usize) -> usize {
        (child - 1) / 2
    }

    /// Returns the indexes of the children of the node at the given index.
    #[inline]
    fn children_indexes(index: usize) -> (usize, usize) {
        let first = index * 2 + 1;
        (first, first + 1)
    }
}

// Utilities
impl<T: Ord> Heap<T> {
    /// Checks whether the element on the left should be put higher on the heap
    /// than the element on the right.
    #[inline]
    fn is_higher(a: &T, b: &T) -> bool {
        a > b
    }

    /// Logically removes the first (maximum) element, putting it on the inner
    /// buffer's end, so that it may be _popped_ by high level callers.
    fn logical_remove(xs: &mut [T]) -> bool {
        if xs.is_empty() {
            return false;
        }

        let last = xs.len() - 1;
        xs.swap(0, last);

        let mut parent = 0;
        while let Some(child) = Self::highest_child(&xs[..last], parent) {
            if Self::is_higher(&xs[child], &xs[parent]) {
                xs.swap(child, parent);
                parent = child;
            } else {
                break;
            }
        }

        true
    }

    /// Returns the highest children of the node at the given index.
    fn highest_child(xs: &[T], parent: usize) -> Option<usize> {
        if !Self::has_any_children(xs.len(), parent) {
            return None;
        }

        let l = 2 * parent + 1;
        let r = l + 1;
        let has_right_child = r < xs.len();

        if !has_right_child || Self::is_higher(&xs[l], &xs[r]) {
            Some(l)
        } else {
            // Small semantic note: if both children are equal, the `r` index
            // is returned.
            Some(r)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{printer::HeapPrinter, Heap};

    macro_rules! heap {
        ($data:expr) => {
            heap!($data, |x| x)
        };
        ($data:expr, $map:expr) => {{
            let mut h = Heap::new();
            for i in [2, 4, 5, 1, 3].into_iter().map($map) {
                h.insert(i);
            }
            h
        }};
    }

    #[test]
    fn test_heap_sort() {
        let h = heap!([2, 4, 5, 1, 3]);

        let s = h.sorted();
        assert_eq!(s, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_heap_sort_desc() {
        use std::cmp::Reverse;
        let h = heap!([2, 4, 5, 1, 3], |x| Reverse(x));

        let s: Vec<_> = h.sorted().into_iter().map(|Reverse(x)| x).collect();
        assert_eq!(s, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_pretty_print() {
        let mut h = Heap::new();
        let mut f = Vec::new();
        for i in [2, 4, 6, 1, 3, 5] {
            h.insert(i);
            f.push(HeapPrinter(&h).to_string());
        }
        assert_eq!(
            f,
            vec![
                "2\n",
                "4\n    2\n",
                "6\n    2\n    4\n",
                "6\n    2\n        1\n    4\n",
                "6\n    3\n        1\n        2\n    4\n",
                "6\n    3\n        1\n        2\n    5\n        4\n",
            ]
        );
    }
}
