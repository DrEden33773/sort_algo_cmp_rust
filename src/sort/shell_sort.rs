fn insertion_sort_in_gap<T>(slice: &mut [T], mut gap: usize)
where
    T: PartialOrd,
{
    if gap >= slice.len() {
        return;
    }
    gap = if gap < 1 { 1 } else { gap };
    for from in (0..slice.len()).step_by(gap) {
        for cmp in (gap..=from).rev().step_by(gap) {
            if slice[cmp - gap] > slice[cmp] {
                slice.swap(cmp - gap, cmp);
            }
        }
    }
}

/// shell_sort
pub fn shell_sort<T: PartialOrd>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }
    insertion_sort_in_gap(slice, 7);
    insertion_sort_in_gap(slice, 5);
    insertion_sort_in_gap(slice, 3);
    insertion_sort_in_gap(slice, 1);
}
