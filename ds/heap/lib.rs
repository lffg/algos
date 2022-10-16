use std::marker::PhantomData;

/// Heap printing utilities.
pub mod printer;

mod kind;
pub use kind::{Max, Min};

/// Maximum heap.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Heap<T, K> {
    data: Vec<T>,
    _kind: PhantomData<K>,
}

impl<T, K> Heap<T, K> {
    /// Returns a new heap.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            _kind: PhantomData,
        }
    }

    /// Returns the heap length (quantity of elements).
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the heap is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the "heap depth" at the given index.
    pub fn depth_at(index: usize) -> usize {
        let pos = index + 1;
        (pos as f64).log2().floor() as usize + 1
    }

    /// Returns the depth of this heap.
    pub fn depth(&self) -> usize {
        if self.len() == 0 {
            0
        } else {
            Self::depth_at(self.len() - 1)
        }
    }

    /// Returns the value at the given index.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    /// Returns an optional reference to the root element.
    pub fn root(&self) -> Option<&T> {
        self.get(0)
    }

    /// Checks if the given index exists and, if so, returns its corresponding
    /// parent index. Otherwise, returns the "parent index location" if the
    /// child were to exist.
    ///
    /// Returns none if the provided index is zero (i.e. the root).
    pub fn parent_index(&self, index: usize) -> Option<Result<usize, usize>> {
        (index != 0).then(|| {
            let loc = (index - 1) / 2;
            if index < self.len() {
                Ok(loc)
            } else {
                Err(loc)
            }
        })
    }

    /// Returns the parent of the provided node index, if such node and its
    /// parent exist.
    pub fn parent(&self, index: usize) -> Option<&T> {
        self.parent_index(index)?
            // SAFETY: If the child exists (`Ok` variant), so does its parent.
            .map(|parent| unsafe { self.data.get_unchecked(parent) })
            .ok()
    }

    /// Returns the children indexes.
    pub fn children_indexes(&self, index: usize) -> (usize, usize) {
        let first = index * 2 + 1;
        (first, first + 1)
    }

    /// Returns optional references to the children of the given node index.
    pub fn children(&self, index: usize) -> (Option<&T>, Option<&T>) {
        let (a, b) = self.children_indexes(index);
        (self.get(a), self.get(b))
    }

    /// Swaps the child with its parent. If the child and the parent exist,
    /// returns its new index, otherwise no operation is performed.
    pub fn swap_parent(&mut self, index: usize) -> Option<usize> {
        self.parent_index(index)?
            .map(|parent| {
                self.data.swap(parent, index);
                parent
            })
            .ok()
    }
}

impl<T, K> Heap<T, K>
where
    T: Ord,
    K: kind::HeapKind,
{
    fn get_child_parent_pair(&mut self, index: usize) -> Option<(&T, &T)> {
        let parent = self.parent(index)?;
        // SAFETY: If `parent` exists, so does the child.
        let child = unsafe { self.data.get_unchecked(index) };
        Some((child, parent))
    }

    /// Puts the given value and returns the inserted-to index.
    fn put(&mut self, value: T) -> usize {
        self.data.push(value);
        // This will never overflow:
        self.len() - 1
    }

    /// Inserts the given element on its appropriate heap position.
    pub fn insert(&mut self, value: T) {
        let mut child_i = self.put(value);
        while let Some((child, parent)) = self.get_child_parent_pair(child_i) {
            if !K::should_swap(child, parent) {
                break;
            }
            // SAFETY: If one has just compared the child with its parent, both
            // must exist (otherwise `get_child_parent_pair` would have returned
            // `None`). Hence, the `unwrap` will never panic.
            child_i = self.swap_parent(child_i).unwrap();
        }
    }
}
