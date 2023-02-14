#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

mod ownership_examples;
mod borrowing_examples;
mod lifetime_examples;
mod smart_pointers_examples;

mod multithreading_examples;

mod cpp_examples;
fn main() {
    
}











#[derive(Debug)]
struct Rectangle {
    x : i32,
    y : i32
}

fn return_square(size : i32) -> Rectangle {
    Rectangle { x: size, y: size }
}

// fn main() {
//     let r = return_square(4);

//     println!("{:?}", r);
// }






