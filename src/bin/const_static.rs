//Two types of constants
// const: An unchangeable value
/* static: A possibly 'mut' able value with 'static' lifetime. The static lifetime is
inferred and does not have to be specified. Accesing or modifying a mutable static
variable is 'unsafe' */

//Globals are declared outside all other scopes
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;
//Constants MUST have an specified type, the compiler wont help you in this one

fn is_big(n: i32) -> bool {
    //Access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    //Acess constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    //Error! you cannot modify a const
    //THRESHOLD = 5; //-> This will give a mistake
}

//Personal
/*"if is_big(n) { "big" } else { "small" }"
single line conditional */