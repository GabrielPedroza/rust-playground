#![allow(dead_code)]
pub fn iters() {
    let arr = [1, 2, 3 ,4 ,5];

    for i in arr.iter() {
        println!("The value is {}", i);
    }

    let mut arr_1 = arr.iter();

    println!("The value is {:?}", arr_1.next());

    let mut arr_iter = arr_1.into_iter(); // can't use arr_1 after this

    println!("The value is {:?}", arr_iter.next());
}