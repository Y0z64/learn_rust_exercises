//Enum can also be used in a C-like manner
#![allow(dead_code)]

//enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two
}

//enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    //Using 'use' to manage scoping
    use crate::Number::*;
    use crate::Color::{Red, Blue};

    //'enums' can be cast as integrers
    println!("zero is {}", Zero as i32);
    println!("one is {}", One as i32);
    
    println!("roses are #{:06x}", Red as i32); //Here the crated enums are used
    println!("violets are #{:06x}", Blue as i32); //Here too
}