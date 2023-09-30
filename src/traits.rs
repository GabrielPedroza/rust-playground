#![allow(dead_code)]

pub fn traits() {
    trait Shape {
        fn new (length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Rectangle {
        length: f32,
        width: f32,
    }

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            Rectangle {
                length,
                width,
            }
        }

        fn area(&self) -> f32 {
            self.length * self.width
        }
    }

    let rect = Rectangle::new(3.0, 4.0);
    println!("Area of rectangle is {}", rect.area());
}