#![allow(dead_code)]
pub fn type_cast() {
    let int_u8 = 15;
    let int_u8_2 = 255;

    let int_u16 = (int_u8 as u16) + (int_u8_2 as u16);

    println!("{}", int_u16);
}