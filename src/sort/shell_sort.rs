fn insertion_sort_in_gap<T>(slice: &mut [T], gap: usize)
where
    T: PartialOrd,
{
    for from in (gap..slice.len()).step_by(gap) {
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
    for gap in [7, 5, 3, 1] {
        insertion_sort_in_gap(slice, gap);
    }
}
