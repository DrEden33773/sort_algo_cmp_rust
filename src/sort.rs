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

use std::thread;
use std::time;

#[allow(dead_code)]
pub fn debug_all_sorts(vec: &Vec<usize>) {
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
        // check
        match if_ascending_ordered(&to_sort) {
            true => (),
            false => panic!("Error => {} is failed.", sort_name),
        }
    }
}

fn if_ascending_ordered<T: PartialOrd>(to_check: &[T]) -> bool {
    for i in 0..to_check.len() - 1 {
        if to_check[i] > to_check[i + 1] {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
pub fn benchmark_all_sorts(vec: &Vec<usize>) {
    // start with newline
    println!();
    // all sort algorithms
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
    // benchmark all sort algorithms in multiple threads
    let mut handles = Vec::with_capacity(sort_func_table.len());
    for (sort_name, sort_func) in sort_func_table {
        // get clone
        let to_sort = vec.clone();
        // exec sort
        let handle = thread::spawn(move || {
            let mut to_sort = to_sort;
            let start_time = time::Instant::now();
            sort_func(&mut to_sort);
            let end_time = time::Instant::now();
            let duration = end_time - start_time;
            println!("{} => {}ms.", sort_name, duration.as_millis());
            match if_ascending_ordered(&to_sort) {
                true => (),
                false => panic!("Error => {} is failed.", sort_name),
            }
        });
        // push handle
        handles.push(handle);
    }
    // wait all threads
    for handle in handles {
        handle.join().unwrap();
    }
    // end with newline
    println!();
}
