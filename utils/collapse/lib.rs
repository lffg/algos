pub trait CollapseExt<T> {
    fn collapse(self) -> T;
}

impl<T> CollapseExt<T> for Result<T, T> {
    fn collapse(self) -> T {
        match self {
            Ok(val) => val,
            Err(val) => val,
        }
    }
}
