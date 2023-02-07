mod bubble_sort;
mod heap_sort;
mod insertion_sort;
mod merge_sort;
mod quick_sort;
mod radix_sort;
mod selection_sort;
mod shell_sort;

use bubble_sort::bubble_sort;
use heap_sort::heap_sort;
use insertion_sort::insertion_sort;
use merge_sort::merge_sort;
use quick_sort::quick_sort;
use radix_sort::radix_sort;
use selection_sort::selection_sort;
use shell_sort::shell_sort;

#[allow(dead_code)]
pub fn debug_all_sorts(vec: &Vec<usize>) {
    use std::time;
    println!();
    let sort_func_table: Vec<(&str, fn(&mut [usize]))> = vec![
        ("quick_sort", quick_sort),
        ("heap_sort", heap_sort),
        ("merge_sort", merge_sort),
        ("radix_sort", radix_sort),
        ("shell_sort", shell_sort),
        ("selection_sort", selection_sort),
        ("insertion_sort", insertion_sort),
        ("bubble_sort", bubble_sort),
    ];
    for (sort_name, sort_func) in sort_func_table {
        // get clone
        let mut to_sort = vec.clone();
        // start time
        let start_time = time::Instant::now();
        // exec sort
        println!("before => {:?}", &to_sort);
        sort_func(&mut to_sort);
        println!("after => {:?}", &to_sort);
        // end time
        let end_time = time::Instant::now();
        // get duration
        let duration = end_time - start_time;
        // println
        println!("{} => {}ms.\n", sort_name, duration.as_millis());
    }
}

#[allow(dead_code)]
pub fn benchmark_all_sorts(vec: &Vec<usize>) {
    use std::time;
    println!();
    let sort_func_table: Vec<(&str, fn(&mut [usize]))> = vec![
        ("quick_sort", quick_sort),
        ("heap_sort", heap_sort),
        ("merge_sort", merge_sort),
        ("radix_sort", radix_sort),
        ("shell_sort", shell_sort),
        ("selection_sort", selection_sort),
        ("insertion_sort", insertion_sort),
        ("bubble_sort", bubble_sort),
    ];
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap();
    for (sort_name, sort_func) in sort_func_table {
        pool.install(|| {
            // get clone
            let mut to_sort = vec.clone();
            // start time
            let start_time = time::Instant::now();
            // exec sort
            sort_func(&mut to_sort);
            // end time
            let end_time = time::Instant::now();
            // get duration
            let duration = end_time - start_time;
            // println
            println!("{} => {}ms.\n", sort_name, duration.as_millis());
        });
    }
}
