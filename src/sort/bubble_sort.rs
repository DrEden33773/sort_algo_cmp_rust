/// bubble_sort
pub fn bubble_sort<T: PartialOrd>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }
    for upper_bond in 0..slice.len() {
        let mut if_ordered = true;
        for cmp in 0..slice.len() - 1 - upper_bond {
            if slice[cmp] > slice[cmp + 1] {
                if_ordered = false;
                slice.swap(cmp, cmp + 1);
            }
        }
        if if_ordered {
            break;
        }
    }
}
