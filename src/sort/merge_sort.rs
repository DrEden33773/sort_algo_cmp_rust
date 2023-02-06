/// merge_sort

fn merge<T>(vec: &mut Vec<T>, begin: usize, end: usize)
where
    T: PartialOrd + Copy,
{
    let mut temp: Vec<T> = Vec::new();

    let mid = (begin + end) / 2;
    let mut left = begin;
    let mut right = mid;

    while left < mid && right < end {
        if vec[left] < vec[right] {
            temp.push(vec[left]);
            left += 1;
        } else {
            temp.push(vec[right]);
            right += 1;
        }
    }
    while left < mid {
        temp.push(vec[left]);
        left += 1;
    }
    while right < end {
        temp.push(vec[right]);
        right += 1;
    }

    for index in 0..temp.len() {
        vec[begin + index] = temp[index];
    }
}

fn ms_helper<T>(vec: &mut Vec<T>, begin: usize, end: usize)
where
    T: PartialOrd + Copy,
{
    if begin + 1 < end {
        let mid = (begin + end) / 2;
        ms_helper(vec, begin, mid);
        ms_helper(vec, mid, end);
        merge(vec, begin, end);
    }
}

pub fn merge_sort<T: PartialOrd + Copy>(vec: &mut Vec<T>) {
    if vec.is_empty() {
        return;
    }
    ms_helper(vec, 0, vec.len());
}
