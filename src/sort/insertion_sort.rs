/// insertion_sort
pub fn insertion_sort<T: PartialOrd>(vec: &mut Vec<T>) {
    if vec.len() <= 1 {
        return;
    }
    for from in 1..vec.len() {
        for cmp in (1..=from).rev() {
            if vec[cmp - 1] > vec[cmp] {
                vec.swap(cmp - 1, cmp);
            }
        }
    }
}
