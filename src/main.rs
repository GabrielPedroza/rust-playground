mod arrays;
mod string;
mod tuples;
mod user;
mod type_casting;
mod enums;
mod vectors;
mod functions;
mod hashmap;
mod traits;
mod err_handling;
mod iterators;
mod closures;
mod boxes;
mod threads;

mod restaurant;
// use crate::restaurant::order_food;
fn main() {
    threads::threads();
}

// *************************
// *** Filename: boxes.rs  *
// *************************
// boxes::boxes();

// *************************
// *** Filename: closures.rs  *
// *************************
// closures::closures();

// *************************
// *** Filename: iterators.rs  *
// *************************
// iterators::iters();

// *************************
// *** Filename: err_handling.rs  *
// *************************
// err_handling::err_handling();

// *************************
// *** Filename: traits.rs  *
// *************************
// order_food();

// *************************
// *** Filename: traits.rs  *
// *************************
// traits::traits();

// *************************
// *** Filename: hashmap.rs  *
// *************************
// hashmap::hashmap();

// *************************
// *** Filename: functions.rs  *
// *************************
// functions::functions();

// *** Filename: enums.rs  *
// *************************
// enums::days_of_week();

// *************************
// *** Filename: vectors.rs  *
// *************************
// vectors::vectors();


// *************************
// *** Filename: type_casting.rs
// *************************
// type_casting::type_cast();

// *************************
// *** Filename: string.rs *
// *************************
// string::string();

// *************************
// *** Filename: tuples.rs *
// *************************
// tuples::user_tuple();

// *************************
// *** Filename: arrays.rs *
// *************************
// arrays::print_odd();

// *************************
// *** Filename: user.rs ***
// *************************
// let (name, age) = (user::get_name(), user::get_age());
// let can_drink = user::user_can_drink(age);
// let years_str = if age == 1 { "year" } else { "years" };

// println!(
//     "Hello {}! You are {} {} old. {}",
//     name,
//     age,
//     years_str,
//     if can_drink {
//         "You can drink!"
//     } else {
//         "You can't drink!"
//     }
// );
