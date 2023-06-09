fn merge<T>(slice: &mut [T])
where
    T: PartialOrd + Copy,
{
    let mut temp: Vec<T> = Vec::with_capacity(slice.len());

    let mid = slice.len() / 2;
    let mut left = 0;
    let mut right = mid;

    while left < mid && right < slice.len() {
        let the_less = if slice[left] < slice[right] {
            &mut left
        } else {
            &mut right
        };
        temp.push(slice[*the_less]);
        *the_less += 1;
    }
    while left < mid {
        temp.push(slice[left]);
        left += 1;
    }
    while right < slice.len() {
        temp.push(slice[right]);
        right += 1;
    }

    slice[..temp.len()].copy_from_slice(&temp[..]);
}

/// merge_sort
pub fn merge_sort<T: PartialOrd + Copy>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }
    let mid = slice.len() / 2;
    merge_sort(&mut slice[..mid]);
    merge_sort(&mut slice[mid..]);
    merge(slice);
}
