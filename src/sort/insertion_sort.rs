/// equivalent one (not that modernized)
pub fn _insertion_sort<T: PartialOrd>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }
    for from in 1..slice.len() {
        let mut cmp = from;
        while cmp >= 1 && slice[cmp - 1] > slice[cmp] {
            slice.swap(cmp - 1, cmp);
            cmp -= 1;
        }
    }
}

/// insertion_sort
pub fn insertion_sort<T: PartialOrd>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }
    for from in 1..slice.len() {
        for cmp in (1..=from).rev() {
            if slice[cmp - 1] > slice[cmp] {
                slice.swap(cmp - 1, cmp);
            } else {
                break;
            }
        }
    }
}
