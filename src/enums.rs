#![allow(dead_code)]
pub fn days_of_week() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Day {
        fn is_weekday(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => false,
                _ => true,
            }
        }
    }

    let today = Day::Monday;

    match today {
        Day::Saturday | Day::Sunday => println!("It's the weekend!"),
        _ => println!("It's a weekday!"),
    }

    println!("Is today a weekday? {}", today.is_weekday());
}