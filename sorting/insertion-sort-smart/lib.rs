use collapse::CollapseExt;
use sorter::Sorter;

// https://github.com/jonhoo/orst/blob/master/src/insertionsort.rs
pub struct InsertionSortSmart;

impl Sorter for InsertionSortSmart {
    fn sort<T: Ord>(&self, xs: &mut [T]) {
        sort(xs);
    }
}

fn sort<T: Ord>(xs: &mut [T]) {
    for i in 1..xs.len() {
        let target = xs[..i].binary_search(&xs[i]).collapse();
        xs[target..=i].rotate_right(1);
    }
}

#[cfg(test)]
mod sorter_tests {
    sorter::test!(super::InsertionSortSmart);
}
