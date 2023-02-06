/// bubble_sort

pub fn bubble_sort<T: PartialOrd>(vec: &mut Vec<T>) {
    if vec.len() <= 1 {
        return;
    }
    for upper_bond in 0..vec.len() {
        let mut if_ordered = true;
        for cmp in 0..vec.len() - 1 - upper_bond {
            if vec[cmp] > vec[cmp + 1] {
                if_ordered = false;
                vec.swap(cmp, cmp + 1);
            }
        }
        if if_ordered {
            break;
        }
    }
}
