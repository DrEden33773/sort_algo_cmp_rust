pub fn rust_dafault_sort<T: Ord>(slice: &mut [T]) {
    slice.sort_unstable();
}
