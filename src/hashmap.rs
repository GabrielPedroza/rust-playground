#![allow(dead_code)]

use std::collections::HashMap;

pub fn hashmap() {
    let mut hashmap = HashMap::new();

    hashmap.insert("name", "John");
    hashmap.insert("age", "30");
    hashmap.insert("city", "Miami");

    println!("The hashmap is {:?}", hashmap);

    for (key, value) in &hashmap {
        println!("{}: {}", key, value);
    }

    if hashmap.contains_key("city") {
        let city = hashmap.get("city");

        match city {
            Some(c) if c == &"Miami" => println!("The city is Miami!!!"),
            Some(c) => println!("The city is {}", c),
            None => println!("The city is not found"),
        }
    }
}