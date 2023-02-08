const RADIX: usize = 10;

static mut BUCKETS: Vec<Vec<usize>> = Vec::new();

fn init_buckets() {
    unsafe {
        BUCKETS = Vec::with_capacity(RADIX);
        for _index in 0..RADIX {
            BUCKETS.push(Vec::new());
        }
    }
}

fn get_max_len(vec: &mut [usize]) -> usize {
    let mut max_len: usize = 0;
    for num_ref in vec {
        let mut num = num_ref.clone();
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

fn build_buckets(to_sort: &mut [usize], pos: usize) {
    // pos should >= 1
    for num_ref in to_sort {
        let mut num = num_ref.clone();
        let mut identifier: usize = 0;
        for _current_pos in (1..=pos).rev() {
            identifier = num % RADIX;
            num /= RADIX;
        }
        unsafe {
            BUCKETS[identifier].push(*num_ref);
        }
    }
}

fn extract_from_buckets(to_sort: &mut [usize]) {
    let mut index: usize = 0;
    unsafe {
        for bucket in &BUCKETS {
            for num in bucket {
                to_sort[index] = *num;
                index += 1;
            }
        }
    }
}

fn clean_buckets() {
    unsafe {
        for bucket in &mut BUCKETS {
            bucket.clear();
        }
    }
}

/// radix_sort
///
/// Generic radix_sort is hard to implement.
///
/// Now, `radix_sort` could only receive `Vec<u32>` as the only input.
pub fn radix_sort(to_sort: &mut [usize]) {
    init_buckets();
    let max_len = get_max_len(to_sort);
    for pos in 1..=max_len {
        build_buckets(to_sort, pos);
        extract_from_buckets(to_sort);
        clean_buckets();
    }
}
