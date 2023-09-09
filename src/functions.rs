#![allow(dead_code)]

use std::ops::Add;

fn get_sum(x: i32, y: i32) -> i32 {
    x + y
}

// fn get_sum(x: i32, y: i32) -> i32 {
//     return x + y;
// }

fn get_sum_x(x: i32) -> (i32, i32) {
    return (x, x + 1);
}

// fn sum_of_list(list: &vec::Vec<i32>) -> i32 {
//     let mut sum = 0;
//     for i in list {
//         sum += i;
//     }
//     sum
// }

fn sum_of_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for i in list {
        sum += i;
    }
    sum
}

// fn sum_generics<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
//     x + y
// }

fn sum_generics<T:Add<Output = T>>(x: T, y: T) -> T {
    x + y
}


pub fn functions() {
    let vec = vec![1, 3, 4, 5, 6, 7, 8, 9];
    let _result: i32 = get_sum(5, 4);
    let (_one, _two) = get_sum_x(5);
    println!("Sum: {}", sum_of_list(&vec));
    println!("Sum: {}", sum_generics(5, 4));
    println!("Sum: {}", sum_generics(5.2, 4.3));
}