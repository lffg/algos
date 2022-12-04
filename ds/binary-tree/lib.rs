use std::cmp::{self, Ordering};

pub mod printer;

pub struct BinaryTree<T> {
    // I don't like this `Option` here, but I still think it is better than
    // having two separate structs, i.e., a `Node` and a `BinaryTree`, which
    // wraps around an optional `Node`.
    value: Option<T>,
    left: Option<Box<BinaryTree<T>>>,
    right: Option<Box<BinaryTree<T>>>,
}

impl<T> BinaryTree<T> {
    /// Creates a new binary search tree.
    pub fn new() -> Self {
        Default::default()
    }

    /// Checks if the BST is empty.
    pub fn is_empty(&self) -> bool {
        self.value.is_none()
    }

    /// Returns the tree's depth.
    pub fn depth(&self) -> usize {
        let l = self.left.as_deref().map(Self::depth).unwrap_or_default();
        let r = self.right.as_deref().map(Self::depth).unwrap_or_default();
        cmp::max(l, r) + usize::from(self.value.is_some())
    }
}

impl<T: Ord> BinaryTree<T> {
    /// Inserts the given value on the BST.
    ///
    /// Returns `Err(T)` if the binary tree already contains the given value.
    pub fn insert(&mut self, value: T) -> Result<(), T> {
        let Some(current) = &self.value else {
            self.value = Some(value);
            return Ok(());
        };
        let subtree = match value.cmp(current) {
            Ordering::Equal => return Err(value),
            Ordering::Less => &mut self.left,
            Ordering::Greater => &mut self.right,
        };
        subtree.get_or_insert_with(Default::default).insert(value)
    }

    /// Checks wether the given element exists in the tree.
    pub fn contains(&self, value: &T) -> bool {
        let Some(current) = &self.value else {
            return false;
        };
        let subtree = match value.cmp(current) {
            Ordering::Equal => return true,
            Ordering::Less => &self.left,
            Ordering::Greater => &self.right,
        };
        subtree
            .as_deref()
            .map(|t| t.contains(value))
            .unwrap_or(false)
    }
}

impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        Self {
            value: None,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{printer::BinaryTreePrinter, BinaryTree};

    #[test]
    fn test_pretty_print() {
        let mut h = BinaryTree::new();
        let mut f = Vec::new();
        for i in [2, 4, 6, 1, 3, 5] {
            h.insert(i).unwrap();
            f.push(BinaryTreePrinter(&h).to_string());
        }
        assert_eq!(
            f,
            vec![
                "2\n",
                "2\n    4\n",
                "2\n    4\n        6\n",
                "2\n    1\n    4\n        6\n",
                "2\n    1\n    4\n        3\n        6\n",
                "2\n    1\n    4\n        3\n        6\n            5\n",
            ]
        );
    }

    #[test]
    fn test_depth_empty() {
        assert_eq!(BinaryTree::<()>::new().depth(), 0);
    }

    #[test]
    fn test_depth_balanced() {
        let mut h = BinaryTree::new();
        for i in [4, 2, 6, 1, 3, 5, 7] {
            h.insert(i).unwrap();
        }
        assert_eq!(h.depth(), 3);
    }

    #[test]
    fn test_depth_unbalanced() {
        let mut h = BinaryTree::new();
        for i in [1, 2, 3, 4, 5, 6, 7] {
            h.insert(i).unwrap();
        }
        assert_eq!(h.depth(), 7);
    }

    #[test]
    fn test_contains() {
        let mut h = BinaryTree::new();
        for i in [4, 2, 6, 1, 3, 5, 7] {
            h.insert(i).unwrap();
        }
        for i in [1, 2, 3, 4, 5, 6, 7] {
            assert!(h.contains(&i));
        }
        for i in [-2, -1, 0, 8, 9, 10] {
            assert!(!h.contains(&i));
        }
    }

    #[test]
    fn test_repeated_element() {
        let mut h = BinaryTree::new();
        h.insert(1).unwrap();
        h.insert(2).unwrap();
        assert_eq!(h.insert(2).unwrap_err(), 2);
    }
}
