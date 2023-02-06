fn make_heap<T: PartialOrd>(vec: &mut Vec<T>, root: usize, end: usize) {
    let mut dad = root;
    let mut son = 2 * dad + 1;
    while son < end {
        // 1. try to cmp two sons
        if son + 1 < end && vec[son] < vec[son + 1] {
            son += 1;
        }
        // 2. cmp dad and max_son
        if vec[dad] < vec[son] {
            vec.swap(dad, son);
            dad = son;
            son = 2 * dad + 1;
        } else {
            return;
        }
    }
}

/// heap_sort
pub fn heap_sort<T: PartialOrd>(vec: &mut Vec<T>) {
    if vec.len() <= 1 {
        return;
    }
    let end = vec.len();
    let len = end;
    // find the last node with two leaves
    let mut root = len / 2 - 1;
    // basic heap
    loop {
        make_heap(vec, root, end);
        if root == 0 {
            break;
        }
        root -= 1;
    }
    // adjust
    for end in (1..=len).rev() {
        let back = end - 1;
        vec.swap(0, back);
        make_heap(vec, 0, back);
    }
}
