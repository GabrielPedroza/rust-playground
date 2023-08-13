#![allow(dead_code)] // allow dead code in this file

use std::io;

pub fn get_age() -> u32 {
    loop {
        let mut age = String::new();
        println!("What is your age?");
        io::stdin().read_line(&mut age).expect("Invalid input");

        match age.trim().parse::<u32>() {
            Ok(age) => {
                if age >= 150 {
                    println!("You're probably dead! Enter a real age!");
                    continue;
                }

                return age;
            }
            Err(_) => {
                println!("Please enter a number!");
            }
        }
    }
}

pub fn get_name() -> String {
    loop {
        let mut name = String::new();
        println!("What is your name?");
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");

        let name = name.trim();

        match name.parse::<String>() {
            Ok(name) => {
                if name.len() == 0 {
                    println!("Enter your name!");
                    continue;
                }

                if name.len() < 2 {
                    println!("Please enter a real name!");
                    continue;
                }

                // check if the trimmed name contains any digits
                if name.chars().any(|c| c.is_digit(10)) {
                    println!("Please enter a name without numbers!");
                    continue;
                }

                return name;
            }
            Err(_) => {
                println!("Please enter a real name!");
            }
        }
    }
}

pub fn user_can_drink(age: u32) -> bool {
    const LEGAL_DRINKING_AGE: u32 = 21;

    return age >= LEGAL_DRINKING_AGE;

    // if age >= LEGAL_DRINKING_AGE {
    //     println!("You can drink!");
    // } else {
    //     println!("You can't drink!");
    // }

    // match age.cmp(&LEGAL_DRINKING_AGE) { // & is a reference to the value
    //     std::cmp::Ordering::Less => println!("You can't drink!"),
    //     std::cmp::Ordering::Equal => println!("You can drink!"),
    //     std::cmp::Ordering::Greater => println!("You can drink!"),
    // }

    // match age {
    //     0..=20 => false,
    //     LEGAL_DRINKING_AGE => true,
    //     _ => true, // _ is a catch all
    // }
}
