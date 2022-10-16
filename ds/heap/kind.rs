pub trait HeapKind {
    /// Checks if the child must be swapped with its parent.
    fn should_swap<T: Ord>(child: &T, parent: &T) -> bool;
}

/// Max heap kind.
pub struct Max;

impl HeapKind for Max {
    fn should_swap<T: Ord>(child: &T, parent: &T) -> bool {
        child > parent
    }
}

/// Min heap kind.
pub struct Min;

impl HeapKind for Min {
    fn should_swap<T: Ord>(child: &T, parent: &T) -> bool {
        child < parent
    }
}
