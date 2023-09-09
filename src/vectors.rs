#![allow(dead_code)]
// vectors are arrays that can grow or shrink in size if mutable
// vectors can only store values of the same type
pub fn vectors() {
    let mut vec_1 = vec![1, 2, 3, 4, 5];

    let _vec_2: Vec<i32> = Vec::new();

    vec_1.push(6);

    let _second = &vec_1[1];

    match vec_1.get(2) {

        Some(x) => println!("Item 3 is {}", x),
        None => println!("Sorry, this vector is too short.")
    }

    println!("{:?}", vec_1); // :? prints the value in a debug format, 
    // which is not as user-friendly but is very informative for 
    // debugging and development purposes.

    // println!("{:#?}", vec_1); // pretty format

    for i in &mut vec_1 {
        *i *= 2;
    }

    for i in &vec_1 {
        println!("{}", i);
    }

    println!("{:?}", vec_1.pop());
    println!("{}", vec_1.len());
}