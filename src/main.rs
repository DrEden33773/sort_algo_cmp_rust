use rand::Rng;

mod sort;
mod tool;

#[allow(unused_variables)]
fn main() {
    let mut rng = rand::thread_rng();

    let vec: Vec<usize> = vec![4, 3, 8, 45, 3, 7, 98, 1, 0];

    let to_sort: Vec<usize> = (0..50000)
        .map(|_| rng.gen_range(0..i32::MAX as usize))
        .collect();

    sort::benchmark_all_sorts(&to_sort);
}
