use sorter::Sorter;

pub struct QuickSortAlloc;

impl Sorter for QuickSortAlloc {
    fn sort<T: Ord>(&self, xs: &mut [T]) {
        sort(xs);
    }
}

// https://github.com/jonhoo/orst/blob/master/src/quicksort.rs
fn sort<T: Ord>(xs: &mut [T]) {
    match xs.len() {
        0 | 1 => return,
        2 => {
            if xs[0] > xs[1] {
                xs.swap(0, 1);
            }
            return;
        }
        _ => (),
    }

    let (pivot, rest) = xs.split_first_mut().unwrap();

    // Elements BEFORE `left` are to be considered at the "left portion" of the
    // slice. Elements on the left are smaller than or equal to the pivot.
    let mut left = 0;
    // Elements AFTER `right` are to be considered at the "right portion" of the
    // slice. Elements on the right are greater than the pivot.
    let mut right = rest.len() - 1;

    while left <= right {
        if &rest[left] <= pivot {
            left += 1;
        } else if &rest[right] > pivot {
            if right == 0 {
                break;
            }
            right -= 1;
        } else {
            rest.swap(left, right);
            left += 1;
            if right == 0 {
                break;
            }
            right -= 1;
        }
    }

    // Re-align `left` to account for `pivot` in the beginning.
    let left = left + 1;

    // Place the pivot on it's final position:
    xs.swap(0, left - 1);

    // Split in a way that pivot gets on the `hi` sub-slice.
    let (lo, hi) = xs.split_at_mut(left - 1);
    // Exclude the pivot, since it's already correctly put.
    let hi = &mut hi[1..];

    sort(lo);
    sort(hi);
}

#[cfg(test)]
mod sorter_tests {
    sorter::test!(super::QuickSortAlloc);
}

struct User {
    username: String,
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("username", &self.username)
            .finish()
    }
}
