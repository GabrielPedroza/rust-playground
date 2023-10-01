#![allow(dead_code)]

pub fn closures() {
    let can_vote = |age: i32| -> bool {
        age >= 18
    };

    let vote = can_vote(20);
    println!("Can vote: {}", vote);

    fn use_func<T>(a: i32, b: i32, func: T) -> i32 where T: Fn(i32, i32) -> i32 {
            func(a, b)
    }

    let sum = use_func(5, 4, |x, y| x + y);
    println!("5 + 4 = {}", sum);

    fn mult(x: i32, y: i32) -> i32 { 
        x * y 
    }

    let product = use_func(4, 5, mult);
    println!("4 * 5 = {}", product);
}