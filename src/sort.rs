mod bubble_sort;
mod heap_sort;
mod insertion_sort;
mod merge_sort;
mod quick_sort;
mod selection_sort;
mod shell_sort;

use bubble_sort::bubble_sort;
use heap_sort::heap_sort;
use insertion_sort::insertion_sort;
use merge_sort::merge_sort;
use quick_sort::quick_sort;
use selection_sort::selection_sort;
use shell_sort::shell_sort;

pub fn dbg_all_sorts<T>(vec: &Vec<T>)
where
    T: std::fmt::Display + PartialOrd + Copy,
{
    use crate::tool::println_vec;
    println!();
    let sort_func_table: Vec<(&str, fn(&mut Vec<T>))> = vec![
        ("bubble_sort", bubble_sort),
        ("insertion_sort", insertion_sort),
        ("selection_sort", selection_sort),
        ("shell_sort", shell_sort),
        ("merge_sort", merge_sort),
        ("quick_sort", quick_sort),
    ];
    for (sort_name, sort_func) in sort_func_table {
        // get clone
        let mut to_sort = vec.clone();
        // show info
        println!("`{}` test => ", sort_name);
        println!();
        // exec sort
        println_vec(&mut to_sort);
        sort_func(&mut to_sort);
        println_vec(&mut to_sort);
        // println
        println!();
    }
}
