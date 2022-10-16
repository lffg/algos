pub trait HeapKind {
    /// Checks if the first argument should be located in a higher position if
    /// compared to the second.
    fn is_higher<T: Ord>(a: &T, b: &T) -> bool;
}

/// Max heap kind.
pub struct Max;

impl HeapKind for Max {
    fn is_higher<T: Ord>(a: &T, b: &T) -> bool {
        a > b
    }
}

/// Min heap kind.
pub struct Min;

impl HeapKind for Min {
    fn is_higher<T: Ord>(a: &T, b: &T) -> bool {
        a < b
    }
}
