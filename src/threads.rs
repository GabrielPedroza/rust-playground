#![allow(dead_code)]

// use std::ops::AddAssign;
pub fn threads() {
    pub trait Transaction {
        fn as_f64(&self) -> f64;
    }

    impl Transaction for f64 {
        fn as_f64(&self) -> f64 {
            *self
        }
    }

    impl Transaction for i64 {
        fn as_f64(&self) -> f64 {
            *self as f64
        }
    }

    pub struct Bank {
        balance: f64,
    }

    impl Bank {
        pub fn new() -> Bank {
            Bank { balance: 0.0 }
        }

        pub fn deposit<T: Transaction>(&mut self, amount: T) -> &mut Self {
            self.balance += amount.as_f64();
            self
        }

        pub fn withdraw<T: Transaction>(&mut self, amount: T) -> &mut Self {
            self.balance -= amount.as_f64();
            self
        }

        pub fn balance(&self) -> f64 {
            self.balance
        }
    }

    let mut bank = Bank::new();
    bank.deposit(100.5);
    bank.deposit(25);
    bank.withdraw(100.5);
    bank.withdraw(25);

    println!("Balance: {}", bank.balance());

}