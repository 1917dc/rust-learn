use std::io;

fn main() {
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    // let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("tup 0 is {}", tup.0);

    // Unlike arrays in some other languages, arrays in Rust have a fixed length.

    let arr = [10, 20, 30, 40, 50];
    println!("arr 0 is {}", arr[0]);

    let arr : [i32; 5] = [1, 2, 3, 4, 5];
    println!("arr 0 is {}", arr[0]);

//    let a = [1, 2, 3, 4, 5];
//
//    println!("Please enter an array index.");
//
//    let mut index = String::new();
//
//    io::stdin()
//        .read_line(&mut index)
//        .expect("Failed to read line");
//
//    let index: usize = index
//        .trim()
//        .parse()
//        .expect("Index entered was not a number");
//
//    let element = a[index];
//
//    println!("The value of the element at index {index} is: {element}");

    let mut index = String::new();

    println!("Type a index number on a array of size 5");
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index is not a number.");

    let arr: [i32; 5] = [0, 1, 2, 40, 4];

    println!("The index {index} is {}", arr[index]);
}
