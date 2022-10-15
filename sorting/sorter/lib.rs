/// Represents a sorting algorithm.
pub trait Sorter {
    /// Sorts the provided slice.
    fn sort<T: Ord>(&self, xs: &mut [T]);
}

#[macro_export]
macro_rules! test_specific {
    (
        $sorter:expr,
        [$( ($name:ident, [$($in:expr),*], [$($out:expr),*]), )*]
    ) => {
        $(
            #[test]
            fn $name() {
                let mut i = vec![$($in),*];
                let o = vec![$($out),*];

                use ::sorter::Sorter;
                $sorter.sort(&mut i);

                assert_eq!(&i, &o);
            }
        )*
    }
}

#[macro_export]
macro_rules! test {
    ($sorter:expr) => {
        ::sorter::test_specific!(
            $sorter,
            [
                (t1, [1, 2, 3, 4, 5], [1, 2, 3, 4, 5]),
                (t2, [5, 4, 3, 2, 1], [1, 2, 3, 4, 5]),
                (t3, [1, 2, 5, 3, 4], [1, 2, 3, 4, 5]),
                (t4, [3, 5, 2, 1, 4], [1, 2, 3, 4, 5]),
            ]
        );
    };
}
