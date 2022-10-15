use sorter::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T: Ord>(&self, xs: &mut [T]) {
        sort(xs);
    }
}

fn sort<T: Ord>(xs: &mut [T]) {
    for i in 0..xs.len() {
        for j in 0..xs.len() - (i + 1) {
            if xs[j] > xs[j + 1] {
                xs.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod sorter_tests {
    sorter::test!(super::BubbleSort);
}
