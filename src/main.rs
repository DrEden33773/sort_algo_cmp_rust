mod sort;

#[allow(dead_code)]
fn dbg() {
    use rand::Rng;
    let simple: Vec<usize> = (0..15)
        .map(|_| rand::thread_rng().gen_range(0..100))
        .collect();
    sort::debug_all_sorts(&simple);
}

#[allow(dead_code)]
fn benchmark(times: usize) {
    use rand::Rng;
    println!();
    for sample in 0..times {
        println!("sample {}", sample);
        let to_sort: Vec<usize> = (0..50000)
            .map(|_| rand::thread_rng().gen_range(0..i32::MAX as usize))
            .collect();
        sort::benchmark_all_sorts(&to_sort);
    }
}

#[allow(unused_variables)]
fn main() {
    // dbg();
    benchmark(10);
}
