#![allow(unused_variables)]

//Numeric literals (ints and floats) can be type anotated by sufix

//By default if no constraint exist the compuliler uses
//i32 for ints
//f64 for floats

fn main() {
    //Suffixed literals, their type is known after initialization
    let x = 1_u8;
    let y = 2_u32;
    let z = 3_f32;

    //Unsuffixed literals, their values depend on how they are used
    let i = 1;
    let f = 1.0;

    //using use to manage manual scoping of modules
    use std::mem::{size_of_val};

    // 'size_of_val' returns the size of a variable in bytes
    // remember that 'size_of_val' uses refernces not the actual object
    println!("Size of 'x' in bytes {}", size_of_val(&x));
    println!("Size of 'y' in bytes {}", size_of_val(&y));
    println!("Size of 'z' in bytes {}", size_of_val(&z));
    println!("Size of 'i' in bytes {}", size_of_val(&i));
    println!("Size of 'f' in bytes {}", size_of_val(&f));

    
}