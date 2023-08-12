mod user;
fn main() {
    let name = user::get_name();
    let age: u32 = user::get_age();
    let can_drink = user::user_can_drink(age);
    let years_str = if age == 1 { "year" } else { "years" };

    println!(
        "Hello {}! You are {} {} old. {}",
        name,
        age,
        years_str,
        if can_drink {
            "You can drink!"
        } else {
            "You can't drink!"
        }
    );
}
