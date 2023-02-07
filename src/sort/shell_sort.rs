use crate::sort::insertion_sort;

fn insertion_sort_in_gap<T>(vec: &mut [T], mut gap: usize)
where
    T: PartialOrd,
{
    if gap >= vec.len() {
        return;
    }
    gap = if gap < 1 { 1 } else { gap };
    for from in (0..vec.len()).step_by(gap) {
        for cmp in (gap..=from).rev().step_by(gap) {
            if vec[cmp - gap] > vec[cmp] {
                vec.swap(cmp - gap, cmp);
            }
        }
    }
}

/// shell_sort
pub fn shell_sort<T: PartialOrd>(vec: &mut [T]) {
    if vec.len() <= 1 {
        return;
    }
    insertion_sort_in_gap(vec, 7);
    insertion_sort_in_gap(vec, 5);
    insertion_sort_in_gap(vec, 3);
    insertion_sort_in_gap(vec, 1);
}
