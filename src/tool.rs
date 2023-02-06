use std::fmt::Display;

pub fn print_vec<T: Display>(vec: &mut Vec<T>) {
    print!("[");
    for index in 0..vec.len() {
        print!("{}", vec[index]);
        if index != vec.len() - 1 {
            print!(", ");
        }
    }
    print!("]");
}

pub fn println_vec<T: Display>(vec: &mut Vec<T>) {
    print_vec(vec);
    println!();
}
