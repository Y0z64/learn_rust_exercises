/* Rust provides no implicit type conversion (coercion) 
between primitive types. But, explicit type conversion 
(casting) can be performed using the as keyword.*/

//Supress warnings for overflowing
#![allow(overflowing_literals)]

//Personal note
/*Casts allow us to explicitly specify, declare or change the type of a variable/value
etc. Its kind of like using '(float)var' in C++ */
/*Usually you will be warned when a type is overflowed by its value but in this case
we specified not to do that for the example */

fn main(){
    let decimal = 65.4321_f32;

    //Error! Rust has no implicit conversion (unlike javascript)
    //let integrer: u8 = decimal; //mismatched types error

    //Explicit conversion
    let integer = decimal as u8;
    let character = integer as char; //I believe this used ASCII values or Utf-8

    //There are limitations to this as a float cannot be converted to a char directly
    //let character_float = decimal as char; //only 'u8' can be cast as 'char' not 'f32'

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    //when casting any value to an usigned type, T
    //T::MAX + 1 is added or substracted until the value fits
    //into the new type

    //1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16); //You can use casting in values directly not only to define or change the types of vars

    //100 - 256 - 256 - 256 = 232
    //Under the hood, the first 8 least sigmificant bits (LSB) are kept,
    //while the rest towards the most sigmificant bit (MSB) get truncated.
    println!("1000 as a u8 is : {}", 1000 as u8); //-> 232

    // -1 + 256 = 255
    println!(" -1 as a u8 is : {}", (-1i8) as u8); //-> 255

    //For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256); // -> 232

    //When casting to a signed type, the (bitwise) result is the same as //Signed types refer to ints
    //first casting to the corresponding unsigned type. If the most significant
    //bit of that value is 1, then the value is negative
    
    //Unless it already fits of course
    println!(" 128 as a i16 is: {}", 128 as i16); //-> 128

    // 128 as u8 -> 128, whose value in the 8-bit two's complement representation is:
    println!(" 128 as a i8 is : {}", 128 as i8);  //-> -128

    //repeating the example above
    //1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8); //-> 232
    // and the value of 232 in 8-bit two's complement representation is -24 
    println!(" 232 as a u8 is : {}", 232 as i8); // -> -24

    /*Since Rust 1.45, the 'as' keyword performs a *saturating cast*
    when casting from float to int. If the floating point value exceeds
    the upper bound or is less than the lower bound, the returned value
    will be equal to the bound crossed. */

    //Personal note
    /*This basically translates to, if  we try to cast a float as an int or u,
    it will ignore everything after the floating point, and if the value is directly
    bigger than the bounds of the type, it will convert it to the nearest valid value*/

    // 300.0 as u8 is 255
    println!("300.0 as u8 is: {}", 300.0_f32 as u8); //since in numbers the '_' will be ignored, we can use it to use typing by sufix, its more redable in my opinion
    //-100.0 as u8 is 0
    println!("-100 as u8 is: {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!(" nan as u8 is : {}", f32::NAN as u8);

    //This behaviour incurs a small runtime cost and can be avoided with safe
    //with unsafe methods, however the results might overflow and
    //return **unsound values**. Use these methods wisely:
    unsafe{
        // 300.0 as u8 is 44
        println!(" 300.0 us u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is : {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());

        /*Wonder if I can 'use' those values in to_int_unchecked, but for some reason
        it feels wrong so I wont do it */
    }
}