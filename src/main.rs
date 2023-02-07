mod sort;
mod tool;

fn slice_receiver<T>(slice: &mut [T]) {}

#[allow(unused_variables)]
fn main() {
    use rand::Rng;

    let to_sort: Vec<usize> = (0..50000)
        .map(|_| rand::thread_rng().gen_range(0..i32::MAX as usize))
        .collect();

    sort::benchmark_all_sorts(&to_sort);
}
