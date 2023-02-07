/// selection_sort  
pub fn selection_sort<T: PartialOrd>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }
    for begin in 0..slice.len() {
        let mut min_index = begin;
        for cmp in begin + 1..slice.len() {
            if slice[min_index] > slice[cmp] {
                min_index = cmp;
            }
        }
        slice.swap(begin, min_index);
    }
}
