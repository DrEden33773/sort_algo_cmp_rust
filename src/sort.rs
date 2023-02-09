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

use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::time;

fn if_ascending_ordered<T: PartialOrd>(to_check: &[T]) -> bool {
    for (left, right) in to_check.windows(2).map(|s| (&s[0], &s[1])) {
        if left > right {
            return false;
        }
    }
    true
}

pub fn debug_all_sorts(vec: &Vec<usize>) {
    println!();
    let sort_func_table: Vec<(&str, fn(&mut [usize]))> = vec![
        ("bubble_sort", bubble_sort),
        ("selection_sort", selection_sort),
        ("insertion_sort", insertion_sort),
        ("shell_sort", shell_sort),
        ("merge_sort", merge_sort),
        ("heap_sort", heap_sort),
        ("radix_sort", radix_sort),
        ("quick_sort", quick_sort),
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
        // give feed back info
        match if_ascending_ordered(&to_sort) {
            true => println!("{} => {}ms.\n", sort_name, duration.as_millis()),
            false => println!("{} => failed.\n", sort_name),
        };
    }
}

pub fn benchmark_all_sorts(vec: &Vec<usize>) -> HashMap<&'static str, time::Duration> {
    // start with newline
    println!();
    // all sort algorithms
    let sort_func_table: Vec<(&str, fn(&mut [usize]))> = vec![
        ("bubble_sort", bubble_sort),
        ("selection_sort", selection_sort),
        ("insertion_sort", insertion_sort),
        ("shell_sort", shell_sort),
        ("merge_sort", merge_sort),
        ("heap_sort", heap_sort),
        ("radix_sort", radix_sort),
        ("quick_sort", quick_sort),
    ];
    // message channel
    let (_tx, rx) = mpsc::channel();
    // benchmark all sort algorithms in multiple threads
    for (sort_name, sort_func) in sort_func_table {
        // get clone of tx
        let tx = _tx.clone();
        // get clone of vec
        let copy_of_vec = vec.clone();
        // exec sort
        thread::spawn(move || {
            let mut to_sort = copy_of_vec; // won't copy, only move
            let start_time = time::Instant::now();
            sort_func(&mut to_sort);
            let end_time = time::Instant::now();
            let duration = end_time - start_time;
            let (_, duration) = match if_ascending_ordered(&to_sort) {
                true => {
                    println!("{} => {}ms.", sort_name, duration.as_millis());
                    (sort_name, Some(duration))
                }
                false => {
                    println!("{} => failed.", sort_name);
                    (sort_name, None)
                }
            };
            // send the message
            tx.send((sort_name, duration)).unwrap();
        });
    }
    // (_tx, rx)'s _tx => never used, only been cloned, won't use again
    // We should drop it, or the channel will never close.
    drop(_tx);
    // get `name_duration_hash_map`
    let name_duration_map = rx
        .iter()
        .filter(|(_, duration)| duration.is_some())
        .map(|(name, duration)| (name, duration.unwrap()))
        .collect::<HashMap<_, _>>();
    // end with newline
    println!();
    // return the map
    name_duration_map
}
