//If - else

//The only diference in Rust is that the conditional does not have to be inside parentesis
//And all branches must return the same type

fn main() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is postive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increate ten-fold");

            //This expression returns an 'i32' type
            10 * n //Notice there is no ';' since the Result is being asigned in this block
        } else {
            println!(", and is a big number, halve the number");

            //This expression must return an 'i32' as well
            n / 2
            //TODO ^ try suppressing the expression with a semicolon.
            //It will return a value of type '()' and since the first expression
            //returns an 'i32' it will not compile
        };
    //   ^ You still have to put a semicolon here! All 'let' bindings need it.

    println!("{} -> {}", n, big_n);
}