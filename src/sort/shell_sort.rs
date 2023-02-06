/// shell_sort
use crate::sort::insertion_sort;

fn insertion_sort_with_interval<T>(vec: &mut Vec<T>, interval: usize)
where
    T: PartialOrd,
{
    if interval >= vec.len() {
        return;
    }
    if interval <= 1 {
        insertion_sort(vec);
        return;
    }
    for begin in 0..interval {
        for from in (begin..vec.len()).step_by(interval) {
            for cmp in (begin + 1..=from).rev().step_by(interval) {
                if vec[cmp - 1] > vec[cmp] {
                    vec.swap(cmp - 1, cmp);
                }
            }
        }
    }
}

pub fn shell_sort<T: PartialOrd>(vec: &mut Vec<T>) {
    if vec.len() <= 1 {
        return;
    }
    insertion_sort_with_interval(vec, 7);
    insertion_sort_with_interval(vec, 5);
    insertion_sort_with_interval(vec, 3);
    insertion_sort_with_interval(vec, 1);
}
