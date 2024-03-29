const RADIX: usize = 10;

type BaseType = usize;

type BaseBucketsRef<'a> = &'a mut [Vec<usize>];

fn create_buckets() -> Vec<Vec<usize>> {
    (0..RADIX).map(|_| Vec::new()).collect()
}

fn get_max_len(vec: &mut [usize]) -> usize {
    let mut max_len: usize = 0;
    for num_ref in vec {
        let mut num = *num_ref;
        let len = {
            let mut len = 0;
            while num > 0 {
                len += 1;
                num /= RADIX;
            }
            len
        };
        max_len = if len > max_len { len } else { max_len }
    }
    max_len
}

fn build_buckets(to_sort: &mut [usize], pos: usize, buckets: BaseBucketsRef) {
    // pos should >= 1
    for num_ref in to_sort {
        let mut num = *num_ref;
        let mut identifier = 0;
        for _current_pos in (1..=pos).rev() {
            identifier = num % RADIX;
            num /= RADIX;
        }
        buckets[identifier].push(*num_ref);
    }
}

fn extract_from_buckets(to_sort: &mut [usize], buckets: BaseBucketsRef) {
    let mut index: usize = 0;
    {
        for bucket in buckets {
            for num in bucket {
                to_sort[index] = *num;
                index += 1;
            }
        }
    }
}

fn clean_buckets(buckets: BaseBucketsRef) {
    for bucket in buckets {
        bucket.clear();
    }
}

/// # radix_sort
///
/// ## for `&mut [u64]`
///
/// Generic radix_sort is hard to implement.
///
/// So we could implement `usize_radix_sort` first, then implement `RadixSortable` for `Vec<type>`
///
/// ### `type` could be:
///
/// - `i8`
/// - `i16`
/// - `i32`
/// - `i64`
/// - `isize`
///
/// **and**
///
/// - `u8`
/// - `u16`
/// - `u32`
/// - `u64`
/// - `usize`
pub fn usize_radix_sort(to_sort: &mut [usize]) {
    let mut buckets: Vec<Vec<usize>> = create_buckets();
    let max_len = get_max_len(to_sort);
    for pos in 1..=max_len {
        build_buckets(to_sort, pos, &mut buckets);
        extract_from_buckets(to_sort, &mut buckets);
        clean_buckets(&mut buckets);
    }
}

pub trait RadixSortable {
    fn radix_sort(&mut self);
}

macro_rules! impl_radix_sortable_for_unsigned {
    ($($type:ty),*) => {
        $(
            impl RadixSortable for [$type] {
                fn radix_sort(&mut self) {
                    let mut to_sort = self
                        .iter()
                        .map(|&num| num as BaseType)
                        .collect::<Vec<_>>();
                    usize_radix_sort(&mut to_sort);
                    for (index, &num) in to_sort.iter().enumerate() {
                        self[index] = num as $type;
                    }
                }
            }
        )*
    };
}

macro_rules! impl_radix_sortable_for_signed {
    ($($type:ty),*) => {
        $(
            impl RadixSortable for [$type] {
                fn radix_sort(&mut self) {
                    let mut non_negatives = self
                        .iter()
                        .filter_map(|&num| if num >= 0 { Some(num as BaseType) } else { None })
                        .collect::<Vec<_>>();
                    let mut negatives = self
                        .iter()
                        .filter_map(|&num| if num < 0 { Some(-num as BaseType) } else { None })
                        .collect::<Vec<_>>();
                    usize_radix_sort(&mut non_negatives);
                    usize_radix_sort(&mut negatives);
                    negatives.reverse();
                    for (index, &num) in negatives.iter().enumerate() {
                        self[index] = -(num as $type);
                    }
                    for (index, &num) in non_negatives.iter().enumerate() {
                        self[index + negatives.len()] = num as $type;
                    }
                }
            }
        )*
    };
}

macro_rules! impl_radix_sortable_for_usize {
    () => {
        impl RadixSortable for [usize] {
            fn radix_sort(&mut self) {
                usize_radix_sort(self);
            }
        }
    };
}

impl_radix_sortable_for_signed!(i8, i16, i32, i64, isize);
impl_radix_sortable_for_unsigned!(u8, u16, u32, u64);
impl_radix_sortable_for_usize!();

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sort::if_ascending_ordered;
    use rand::Rng;

    macro_rules! test_radix_sort_for_signed_with_type {
        ($($type:ty),*) => {
            $({
                let mut vec = (0..100)
                    .map(|_| rand::thread_rng().gen_range(-100..100))
                    .collect::<Vec<$type>>();
                vec.radix_sort();
                assert!(if_ascending_ordered(&vec));
            })*
        };
    }

    macro_rules! test_radix_sort_for_unsigned_with_type {
        ($($type:ty),*) => {
            $({
                let mut vec = (0..100)
                    .map(|_| rand::thread_rng().gen_range(0..100))
                    .collect::<Vec<$type>>();
                vec.radix_sort();
                assert!(if_ascending_ordered(&vec));
            })*
        };
    }

    #[test]
    fn test_usize_radix_sort() {
        let mut vec = (0..100)
            .map(|_| rand::thread_rng().gen_range(0..100))
            .collect::<Vec<usize>>();
        usize_radix_sort(&mut vec);
        assert!(if_ascending_ordered(&vec));
    }

    #[test]
    fn test_radix_sort() {
        test_radix_sort_for_signed_with_type!(i8, i16, i32, i64, isize);
        test_radix_sort_for_unsigned_with_type!(u8, u16, u32, u64, usize);
    }
}
