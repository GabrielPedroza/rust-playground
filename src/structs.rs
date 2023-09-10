#![allow(dead_code)]

pub fn structs() {
    struct Customer<T> {
        name: String,
        age: u8,
        address: String,
        balance: T,
    }

    let customer1 = Customer {
        name: String::from("John"),
        age: 30,
        address: String::from("Miami"),
        balance: 100.00,
    };

    let customer2 = Customer {
        name: String::from("Mary"),
        age: 25,
        address: String::from("New York"),
        balance: 200,
    };

    customer2.address = String::from("New Jersey");
}