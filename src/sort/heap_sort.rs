fn make_heap<T: PartialOrd>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }
    let root = 0;
    let end = slice.len() - 1;
    let mut dad = root;
    let mut son = 2 * dad + 1;
    while son < end {
        // 1. try to cmp two sons
        if son + 1 < end && slice[son] < slice[son + 1] {
            son += 1;
        }
        // 2. cmp dad and max_son
        if slice[dad] < slice[son] {
            slice.swap(dad, son);
            dad = son;
            son = 2 * dad + 1;
        } else {
            return;
        }
    }
}

/// heap_sort
pub fn heap_sort<T: PartialOrd>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }
    // find the last node with two leaves
    let last_root = slice.len() / 2 - 1;
    // basic heap
    for root in (0..=last_root).rev() {
        make_heap(&mut slice[root..]);
    }
    // adjust
    for end in (1..=slice.len()).rev() {
        let back = end - 1;
        slice.swap(0, back);
        make_heap(&mut slice[..back]);
    }
}
