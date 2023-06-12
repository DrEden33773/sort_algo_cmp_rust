use rand::Rng;

#[allow(dead_code)]
fn legacy_partition<T>(slice: &mut [T]) -> usize
where
    T: PartialOrd + Copy,
{
    let mut front = 0;
    let mut back = slice.len() - 1;
    let mut pivot = front;
    let pivot_val = slice[pivot];
    while front < back {
        while front < back && slice[back] >= pivot_val {
            back -= 1;
        }
        slice[front] = slice[back];
        while front < back && slice[front] <= pivot_val {
            front += 1;
        }
        slice[back] = slice[front];
    }
    // assert_eq!(front, back);
    slice[front] = pivot_val;
    pivot = front;
    pivot
}

#[allow(dead_code)]
fn partition<T>(slice: &mut [T]) -> usize
where
    T: PartialOrd + Copy,
{
    let front = 0;
    let back = slice.len() - 1;
    let mut pivot = front;
    slice.swap(pivot, back);
    let mut eof_less = front; // [begin, eof_less) contain 0 element at first
    let mut eof_greater = front; // [eof_less, eof_great) contain 0 element at first
    while eof_greater < back {
        if slice[eof_greater] < slice[back] {
            slice.swap(eof_greater, eof_less);
            eof_less += 1;
        }
        eof_greater += 1;
    }
    slice.swap(eof_less, back);
    pivot = eof_less;
    pivot
}

/// ## Modern Partition
///
/// ### Randomized
///
/// This `partition` procedure has been `randomized`,
/// with the worst case of `T(n) = O(nlogn)`
#[allow(dead_code)]
fn modern_partition<T>(slice: &mut [T]) -> usize
where
    T: PartialOrd + Copy,
{
    let mut pivot: isize = -1;
    let len = slice.len();
    let temp = rand::thread_rng().gen_range(0..len);
    slice.swap(temp, len - 1);
    for curr in 0..len {
        if slice[curr] <= slice[len - 1] {
            pivot += 1;
            slice.swap(pivot as usize, curr);
        }
    }
    pivot as usize
}

/// quick_sort
pub fn quick_sort<T: PartialOrd + Copy>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }
    let pivot = modern_partition(slice);
    quick_sort(&mut slice[..pivot]);
    quick_sort(&mut slice[pivot + 1..]);
}

/// ## Quick Select
///
/// ### Randomized
///
/// with the `worst` case of `T(n) = O(n)`
#[allow(dead_code)]
pub fn quick_select<T>(slice: &[T], nth: usize) -> Option<T>
where
    T: PartialOrd + Clone,
{
    let sep = &slice[rand::thread_rng().gen_range(0..slice.len())];
    let lt = slice.iter().pick_by(|x| x < sep);
    let eq = slice.iter().pick_by(|x| x == sep);
    let gt = slice.iter().pick_by(|x| x > sep);
    let (l, e, g) = (lt.len(), eq.len(), gt.len());
    match nth {
        i if i < l => quick_select(&lt, nth),
        i if i < l + e => Some(eq[0].clone()),
        i if i < l + e + g => quick_select(&gt, nth - l - e),
        _ => None,
    }
}

pub trait PickBy<'a, T: 'a + Clone>: Iterator<Item = &'a T> + Sized {
    fn pick_by<F: Fn(&T) -> bool>(self, f: F) -> Vec<T> {
        self.filter_map(|x| if f(x) { Some(x.clone()) } else { None })
            .collect()
    }
}

impl<'a, T: 'a + Clone, I: Iterator<Item = &'a T>> PickBy<'a, T> for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_select() {
        let mut arr = [1, 3, 2, 5, 4];
        for index in 0..(arr.len() + 1) {
            let ans = quick_select(&arr, index);
            let expected = {
                arr.sort();
                arr.get(index).cloned()
            };
            assert_eq!(ans, expected);
        }
    }
}
