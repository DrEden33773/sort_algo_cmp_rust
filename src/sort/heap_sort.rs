/// heap_sort

fn make_heap<T>(vec: &mut Vec<T>, begin: usize, end: usize)
where
    T: PartialOrd + Copy,
{
}

pub fn heap_sort<T: PartialOrd + Copy>(vec: &mut Vec<T>) {
    if vec.len() <= 1 {
        return;
    }
    let end = vec.len();
    let len = end;
    let root = len / 2 - 1;
}
