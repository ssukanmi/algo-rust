use utils::arrays;

pub mod common;
pub mod easy;
pub mod hard;
pub mod medium;
pub mod utils;

fn main() {
    println!("Hello, world!");
    let array = vec![3, 5, -4, 8, 11, 1, -1, 6];
    println!("Array: {:?}", array);
    println!("Array length: {}", array.len());
    println!("Array capacity: {}", array.capacity());

    for i in 0..array.len() {
        println!("Array[{}]: {}", i, array[i]);
    }

    for i in 1..10 {
        println!("Hello, world! {}", i);
    }

    let x: Vec<i32> = arrays::create_array(5);
    println!("Array: {:?}", x);
    let s: Vec<&str> = arrays::create_array(3);
    println!("Array: {:?}", s);

    let x = (6 + 3) / 2;
    println!("x: {}", x);
}
