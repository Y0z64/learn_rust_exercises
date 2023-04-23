// use std::string::{String};
#![allow(dead_code)]
//Usualy type conversion is done using type casting, but for certain types and 
//custom types traits like 'from' and 'into' have to be implemented

//From
//The 'From' Trait allows for a type to define how to create itself from another type
//One example of this train in implementation is:

//We can implement something similar in custom types
// let my_str = "hello";
// let my_string = String::from(my_str);
/*Had to comment this out since you are not suposed to define with let in global
scope */

#[derive(Debug)]
struct Number { //Define custom struct or type
    value: i32,
}

impl From<i32> for Number { //Implement From<i32>
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30); //Create a type Number val from an i32 one
    println!("My number is {:?}", num);
}
//From basically reads A From B


//Into
/*The Into trait is simply the reciprocal of the From trait. 
That is, if you have implemented the From trait for your type, 
Into will call it when necessary.

Using the Into trait will typically require specification of 
the type to convert into as the compiler is unable to determine 
this most of the time. However this is a small trade-off considering 
we get the functionality for free. */

use std::convert::From;

//In this case we already declared Number and implemented From in it
//.into() basically detects to what a value has to convert, with type annotation
//or other way to declare and detect type, and turns it into it

fn main_2() {
    let int = 5;
    //Try removing the type annotation
    let num: Number = int.into();
    println!("My number is {:?}", num);
}