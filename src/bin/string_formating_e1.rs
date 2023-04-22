fn main() {
    println!("{} days", 42);
    println!("{0}, this is {1}. {1}, this is {0}", "Sex", "Cum");

    //define arguments inside the println!
    println!("{subject} {verb} {object}",
            object = "the lazy fox",
            subject="The quick dog",
            verb="jumps over");

    //defining format character inside format! (println! is the same as format but in console output and a new line apended)
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C

    //add right-justify and left-adjust
    println!("{number:0>5}", number=1);

    //use the value of named arguments for formats using $ at the end
    //kind of like with bash but at the end of the argument
    println!("{number:0>width$}", number=1, width=5);

    //The rust compiler checks and warns if the correct number of arguments was not used
    /* println!("My name is {0} {1}", "Slim", /* "Shady" */); */

    #[allow(dead_code)] //disable 'dead_code' which warn against the next struct
    struct Structure(i32);
    /*Allow dead code allows you to atach it to a function or code so that if not
    used it will not show a dead_code warning in the compiler, very cool */

    /* Rust>1.58 can capture the value from
    variables in the same scope, just like any other programming language.
    Not even impresive */
    let number: f64 = 1.0;
    let width: usize =5;
    println!("{number:>width$}");

    //Exersise
    //Display pi with a set number of decimals
    let pi = 3.141592;
    println!("{pi}");
    println!("{pi:.2}");
    /* Didn't even have to look up the documentation wich says a lot */
}