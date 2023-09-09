#![allow(dead_code)]
pub fn string() {
    let str = String::from("A B C D E F G H J J GABE");

    let mut str_chars: Vec<char> = str.chars().collect();
    str_chars.sort();
    str_chars.dedup();

    // for word in &str_chars {
    //     println!("{}", word)
    // };

    let str_2 = "some string"; // A string literal stored in the program's binary
    let mut str_3 = str_2.to_string(); // A new heap-allocated String created from str_2
                                       // str_2 is a reference to the static string literal
                                       // str_3 is a mutable reference to a heap-allocated String

    // println!("{}", str_3);

    let _byte_array = str_3.as_bytes();

    let str_4 = &str_3[0..=5].to_string(); // 5 is inclusive

    // println!("{}", str_4.len());

    str_3.clear();

    println!("{}", str_4);

    let str_5 = String::from("A word");
    let str_6 = String::from(" from my head");

    // these are the same thing
    let str_7 = str_5 + &str_6;
    // let str_7 = str_5 + str_6.as_str();

    println!("{}", str_7);
}

// let mut str = String::new();
// str.push('A');
// str.push_str(" string");

// let str2 = str.replace("A", "An awesome");

// for word in str2.split_whitespace() {
//     println!("{}", word);
// }

// println!("{}", str);
// println!("{}", str2);
