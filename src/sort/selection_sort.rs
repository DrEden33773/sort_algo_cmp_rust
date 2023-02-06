/// selection_sort  

pub fn selection_sort<T: PartialOrd>(vec: &mut Vec<T>) {
    if vec.is_empty() {
        return;
    }
    for begin in 0..vec.len() {
        let mut min_index = begin;
        for cmp in begin + 1..vec.len() {
            if vec[min_index] > vec[cmp] {
                min_index = cmp;
            }
        }
        vec.swap(begin, min_index);
    }
}
