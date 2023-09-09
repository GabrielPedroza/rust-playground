#![allow(dead_code)]
pub fn user_tuple() {
    let tup: (u32, String, f64) = (21, "Gabriel".to_string(), 10_000.00);

    let (age, name, money) = tup;

    println!(
        "My name is {} and I am {} years old. I have {}.",
        name,
        age,
        if money > 1000.00 {
            "lot of money"
        } else {
            "no money"
        }
    );
}
