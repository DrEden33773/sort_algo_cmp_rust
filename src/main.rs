mod sort;
mod tool;

fn main() {
    let vec: Vec<usize> = vec![4, 3, 8, 45, 3, 7, 98, 1, 0];
    sort::dbg_all_sorts(&vec);
}
