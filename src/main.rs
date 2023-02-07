mod sort;
mod tool;

#[allow(unused_variables)]
fn main() {
    use rand::Rng;

    let to_sort: Vec<usize> = (0..50000)
        .map(|_| rand::thread_rng().gen_range(0..i32::MAX as usize))
        .collect();

    let simple: Vec<usize> = (0..15)
        .map(|_| rand::thread_rng().gen_range(0..100))
        .collect();

    // sort::debug_all_sorts(&simple);
    sort::benchmark_all_sorts(&to_sort);
}
