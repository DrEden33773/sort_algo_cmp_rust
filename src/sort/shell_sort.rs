fn insertion_sort_with_gap<T>(slice: &mut [T], gap: usize)
where
    T: PartialOrd,
{
    for from in gap..slice.len() {
        for cmp in (gap..=from).rev().step_by(gap) {
            if slice[cmp - gap] > slice[cmp] {
                slice.swap(cmp - gap, cmp);
            } else {
                break;
            }
        }
    }
}

/// shell_sort
pub fn shell_sort<T: PartialOrd>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }
    let mut gap = slice.len() / 2;
    while gap > 0 {
        insertion_sort_with_gap(slice, gap);
        gap /= 2;
    }
}
