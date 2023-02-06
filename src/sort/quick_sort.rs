#[allow(dead_code)]
fn main_stream_partition<T>(vec: &mut Vec<T>, begin: usize, end: usize) -> usize
where
    T: PartialOrd + Copy,
{
    let mut front = begin;
    let mut back = end - 1;
    let mut pivot = begin;
    let pivot_val = vec[pivot].clone();
    while front < back {
        while front < back && vec[back] >= pivot_val {
            back -= 1;
        }
        vec[front] = vec[back];
        while front < back && vec[front] <= pivot_val {
            front += 1;
        }
        vec[back] = vec[front];
    }
    // assert_eq!(front, back);
    vec[front] = pivot_val;
    pivot = front;
    pivot
}

#[allow(dead_code)]
fn partition<T>(vec: &mut Vec<T>, begin: usize, end: usize) -> usize
where
    T: PartialOrd + Copy,
{
    let front = begin;
    let back = end - 1;
    let mut pivot = begin;
    vec.swap(pivot, back);
    let mut eof_less = front; // [begin, eof_less) contain 0 element at first
    let mut eof_greater = front; // [eof_less, eof_great) contain 0 element at first
    while eof_greater < back {
        if vec[eof_greater] < vec[back] {
            vec.swap(eof_greater, eof_less);
            eof_less += 1;
        }
        eof_greater += 1;
    }
    vec.swap(eof_less, back);
    pivot = eof_less;
    pivot
}

fn qs_helper<T>(vec: &mut Vec<T>, begin: usize, end: usize)
where
    T: PartialOrd + Copy,
{
    if begin + 1 < end {
        /* could use `partition` or `main_stream_partition` */
        let pivot = partition(vec, begin, end);
        /* sorted slice shouldn't contain `vec[pivot]` */
        qs_helper(vec, begin, pivot);
        qs_helper(vec, pivot + 1, end);
    }
}

/// quick_sort
pub fn quick_sort<T: PartialOrd + Copy>(vec: &mut Vec<T>) {
    if vec.len() <= 1 {
        return;
    }
    qs_helper(vec, 0, vec.len());
}
