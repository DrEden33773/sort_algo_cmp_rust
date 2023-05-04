use rand::Rng;
use std::collections::HashMap;
use std::time;

mod sort;

#[allow(dead_code)]
fn debug() {
    let simple: Vec<usize> = (0..15)
        .map(|_| rand::thread_rng().gen_range(0..100))
        .collect();
    sort::debug_all_sorts(&simple);
}

#[allow(dead_code)]
fn benchmark(times: usize, if_add_special_sample: bool) {
    // begin
    println!();
    // special sample
    if if_add_special_sample {
        // descending sample
        {
            println!("descending sample");
            let mut to_sort: Vec<usize> = (0..50000)
                .map(|_| rand::thread_rng().gen_range(0..i32::MAX as usize))
                .collect();
            to_sort.sort_unstable_by(|a, b| b.cmp(a));
            sort::benchmark_all_sorts(&to_sort);
        }
        // ascending sample
        {
            println!("ascending sample");
            let mut to_sort: Vec<usize> = (0..50000)
                .map(|_| rand::thread_rng().gen_range(0..i32::MAX as usize))
                .collect();
            to_sort.sort_unstable();
            sort::benchmark_all_sorts(&to_sort);
        }
    }
    // random sample
    let map_of_average = {
        // map_of_sum
        let mut map_of_sum: HashMap<&str, (usize, time::Duration)> = HashMap::new();
        // benchmark
        for sample in 1..=times {
            // info
            println!("random sample {}", sample);
            // generate random sample
            let to_sort: Vec<usize> = (0..50000)
                .map(|_| rand::thread_rng().gen_range(0..i32::MAX as usize))
                .collect();
            // exec benchmark & add to map_of_sum
            map_of_sum = sort::benchmark_all_sorts(&to_sort).into_iter().fold(
                map_of_sum,
                |mut map, (key, value)| {
                    map.entry(key)
                        .and_modify(|(count, duration)| {
                            *count += 1;
                            *duration += value;
                        })
                        .or_insert((1, value));
                    map
                },
            );
        }
        // get map_of_average
        let map_of_average: HashMap<&str, time::Duration> = map_of_sum
            .into_iter()
            .map(|(key, (count, duration))| (key, duration / count as u32))
            .collect();
        // return
        map_of_average
    };
    // out put average time
    println!("Average time of {} random samples : \n", times);
    for (key, value) in map_of_average {
        println!("{} => {}ms", key, value.as_millis());
    }
    // end
    println!();
}

fn main() {
    // debug();
    benchmark(10, false);
}
