/*!

# Bucket Sort

**First**, implement `bucket_sort_for_01_rv` only suits `&mut [f64]` with each element in (0..1)

**Then**, implement `usize_bucket_sort`, with process below:

1. `&mut [usize]` -> `&mut [f64]` `::` div `max_decimal_digit_len` foreach
2. sort `&mut [f64]` with `bucket_sort_for_01_rv`
3. `&mut [f64]` -> `&mut [usize]` `::` mul `max_decimal_digit_len` foreach

**Finally**, implement `trait BucketSortable: Sized` with `fn bucket_sort(&mut self)`

- `Macros` are in need while dealing with **Finally** step

In the end, you could execute like this:

```rust
use algorithm::sort::bucket_sort::BucketSortable;

let mut to_sort = vec![5, 1, 3, 2, 4];
to_sort.bucket_sort();
assert_eq!(to_sort, vec![1, 2, 3, 4, 5]);
```

*/

#![allow(dead_code)]

use std::error::Error;

fn bucket_sort_for_01_rv(s: &mut [f64]) -> Result<(), Box<dyn Error>> {
    let n = s.len();
    let mut buckets = vec![vec![]; n];
    for &num in s.iter() {
        if !(0.0..1.0).contains(&num) {
            return Err(format!("{num} out of range (0.0 .. 1.0)").into());
        }
        let index = (num * n as f64) as usize;
        buckets[index].push(num);
    }
    for bucket in buckets.iter_mut() {
        bucket.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    }
    let mut index = 0;
    for bucket in buckets.iter() {
        for &num in bucket.iter() {
            s[index] = num;
            index += 1;
        }
    }
    Ok(())
}

fn get_decimal_digits(num: usize) -> i32 {
    let mut num = num;
    let mut digits = 0;
    while num > 0 {
        num /= 10;
        digits += 1;
    }
    digits
}

fn shrink_to_01_range(s: &mut [usize]) -> (Vec<f64>, i32) {
    let mut max_decimal_digit = 0;
    for num in s.iter() {
        max_decimal_digit = max_decimal_digit.max(get_decimal_digits(*num));
    }
    let s_01 = s
        .iter()
        .map(|&num| num as f64 / 10_f64.powi(max_decimal_digit))
        .collect();
    (s_01, max_decimal_digit)
}

fn bucket_sort_helper(to_sort: &mut [usize]) -> Result<(), Box<dyn Error>> {
    let (mut s_01, max_decimal_digit) = shrink_to_01_range(to_sort);
    bucket_sort_for_01_rv(&mut s_01)?;
    for (index, &num) in s_01.iter().enumerate() {
        to_sort[index] = (num * 10_f64.powi(max_decimal_digit)) as usize;
    }
    Ok(())
}

pub fn usize_bucket_sort(to_sort: &mut [usize]) -> Result<(), Box<dyn Error>> {
    if to_sort.is_empty() {
        return Ok(());
    }
    bucket_sort_helper(to_sort)
}

pub fn usize_bucket_sort_adaptive_forward(to_sort: &mut [usize]) {
    if to_sort.is_empty() {
        return;
    }
    bucket_sort_helper(to_sort).unwrap();
}

pub trait BucketSortable {
    fn bucket_sort(&mut self) -> Result<(), Box<dyn Error>>;
}

macro_rules! impl_bucket_sortable_for_unsigned {
    ($($type:ty),*) => {
        $(
            impl BucketSortable for [$type] {
                fn bucket_sort(&mut self) -> Result<(), Box<dyn Error>> {
                    let mut to_sort = self
                        .iter()
                        .map(|&num| num as usize)
                        .collect::<Vec<_>>();
                    usize_bucket_sort(&mut to_sort)?;
                    for (index, &num) in to_sort.iter().enumerate() {
                        self[index] = num as $type;
                    }
                    Ok(())
                }
            }
        )*
    };
}

macro_rules! impl_bucket_sortable_for_signed {
    ($($type:ty),*) => {
        $(
            impl BucketSortable for [$type] {
                fn bucket_sort(&mut self) -> Result<(), Box<dyn Error>> {
                    let mut non_negatives = self
                        .iter()
                        .filter_map(|&num| if num >= 0 { Some(num as usize) } else { None })
                        .collect::<Vec<_>>();
                    let mut negatives = self
                        .iter()
                        .filter_map(|&num| if num < 0 { Some(-num as usize) } else { None })
                        .collect::<Vec<_>>();
                    usize_bucket_sort(&mut non_negatives)?;
                    usize_bucket_sort(&mut negatives)?;
                    negatives.reverse();
                    for (index, &num) in negatives.iter().enumerate() {
                        self[index] = -(num as $type);
                    }
                    for (index, &num) in non_negatives.iter().enumerate() {
                        self[index + negatives.len()] = num as $type;
                    }
                    Ok(())
                }
            }
        )*
    };
}

macro_rules! impl_bucket_sortable_for_usize {
    () => {
        impl BucketSortable for [usize] {
            fn bucket_sort(&mut self) -> Result<(), Box<dyn Error>> {
                usize_bucket_sort(self)
            }
        }
    };
}

impl_bucket_sortable_for_unsigned!(u8, u16, u32, u64);
impl_bucket_sortable_for_signed!(i8, i16, i32, i64, isize);
impl_bucket_sortable_for_usize!();

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sort::if_ascending_ordered;
    use rand::Rng;

    macro_rules! test_bucket_sort_for_signed {
        ($($type:ty),*) => {
            $({
                let mut vec = (0..100)
                    .map(|_| rand::thread_rng().gen_range(-100..100))
                    .collect::<Vec<$type>>();
                vec.bucket_sort().unwrap();
                assert!(if_ascending_ordered(&vec));
            })*
        };
    }

    macro_rules! test_bucket_sort_for_unsigned {
        ($($type:ty),*) => {
            $({
                let mut vec = (0..100)
                    .map(|_| rand::thread_rng().gen_range(0..100))
                    .collect::<Vec<$type>>();
                vec.bucket_sort().unwrap();
                assert!(if_ascending_ordered(&vec));
            })*
        };
    }

    #[test]
    fn test_usize_bucket_sort() {
        let mut vec = (0..100)
            .map(|_| rand::thread_rng().gen_range(0..100))
            .collect::<Vec<usize>>();
        usize_bucket_sort(&mut vec).unwrap();
        assert!(if_ascending_ordered(&vec));
    }

    #[test]
    fn test_bucket_sort() {
        test_bucket_sort_for_signed!(i8, i16, i32, i64, isize);
        test_bucket_sort_for_unsigned!(u8, u16, u32, u64, usize);
    }
}
