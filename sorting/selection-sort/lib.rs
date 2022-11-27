use sorter::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T: Ord>(&self, xs: &mut [T]) {
        sort(xs);
    }
}

fn sort<T: Ord>(xs: &mut [T]) {
    for i in 0..xs.len().saturating_sub(1) {
        let mut min = i;
        for j in (i + 1)..xs.len() {
            if xs[min] > xs[j] {
                min = j;
            }
        }
        xs.swap(min, i);
    }
}

#[cfg(test)]
mod sorter_tests {
    sorter::test!(super::SelectionSort);
}
