mod bubble_sort;
mod insertion_sort;
mod merge_sort;
mod selection_sort;
mod shell_sort;

use bubble_sort::bubble_sort;
use insertion_sort::insertion_sort;
use selection_sort::selection_sort;
use shell_sort::shell_sort;

use crate::tool::println_vec;
use std::fmt::Display;

pub fn dbg_all_sorts<T>(vec: &Vec<T>)
where
    T: Display + PartialOrd + Clone,
{
    println!();
    let sort_func_table: Vec<(&str, fn(&mut Vec<T>))> = vec![
        ("bubble_sort", bubble_sort),
        ("insertion_sort", insertion_sort),
        ("selection_sort", selection_sort),
        ("shell_sort", shell_sort),
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
