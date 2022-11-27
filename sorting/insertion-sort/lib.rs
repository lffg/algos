use sorter::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T: Ord>(&self, xs: &mut [T]) {
        sort(xs);
    }
}

fn sort<T: Ord>(xs: &mut [T]) {
    for i in 1..xs.len() {
        let mut prev = i - 1;
        while xs[prev] > xs[prev + 1] {
            xs.swap(prev, prev + 1);
            if prev == 0 {
                break;
            }
            prev -= 1;
        }
    }
}

#[cfg(test)]
mod sorter_tests {
    sorter::test!(super::InsertionSort);
}
