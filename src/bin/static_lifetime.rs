//Refers to Static lifetime "'static" not the constant definition

// Rust has a few reserved lifetime names. One of those is 'static. You might encounter it in two situations:
// //A reference with 'static lifetime:
// let s: &'static str = "hello world";

// //'static as part of a trait bound
// fn generic<T>(x: T where T: 'static {})

//There are diference in those, explained in here

//---------Reference lifetime
/*As a reference lifetime 'static indicates that the data pointed to by the reference
lives for the entire lifetime of the running program. It can still coerced to a shorter
lifetime. */

//There are two times to make a variable with 'static lifetime:
//Make a constant with static
static NUM: i32 = 18;

//Make a string with with type &'static str
#[allow(dead_code)]
static MY_STRING: &'static str = "Hello, world!";

//Returns a reference to 'NUM' where its 'static
//lifetime is coerced to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        //Make a 'string' literal and print it:
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);
    }

    {
        //Make an integer to use for 'coerce_static':
        let lifetime_num = 9;

        //Coerce 'NUM' to lifetime of 'lifeitime_num'
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accesible!", NUM);
}

/* So like if a value its inside a scope it only has a lifetime inside the scope,
until it is no longer inside it. When you make it static, it will always be avaliable*/

//-------------TRAIT BOUND
use std::fmt::Debug;

#[allow(dead_code)]
fn print_it( input: impl Debug + 'static) {
    println!("'static value passed in is {:?}", input);
}
//In here the function is asking for an input of Debug AND 'static
//Its implementing Debug just like [derive(Debug)] would

// fn main() {
//     // i is owned and contains no references, thus it's 'static:
//     let i = 5;
//     print_it(i);

//     // oops, &i only has the lifetime defined by the scope of
//     //main(), so it's not 'static;
//     //print_it(&i); //This will send an Error!
// }

// //So in the local scope i is 'static but the reference of &i is not
// //and so it will not be printed