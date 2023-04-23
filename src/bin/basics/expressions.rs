#![allow(unused_must_use)]
//Expressions

//Rust programs are (mostly) made of series of statements
/*
fn main() {
    // statement
    // statement
    // statement
}
 */

//There are a few kinds of statements in Rust. The most common two are declaring
//a variable binding, and using a ';' with a expression
/*
    fn main(){
        //variable binding
        let x = 5;

        //expression;
        x;
        x + 1;
        15;
    }
 */

//Blocks are expressions too, so they can be used as values in assignemnets.
//The last expression in the block will be assigned to the place expression such
//as a local variable. However, if the last expression of the block ends wuth a
//semicolon the return value will be ()

fn main(){
    let x = 5_u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        //This expression will be assigned to 'y'
        x_cube + x_squared + x //No ';'
    };

    let z = { //--> You can see the type here is '()'
        //The semicolon supresses this expression and '()' is assigned to 'z'
        2 * x; //';'
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

//This used of blocks for expression definition is why in the 'impl' of 'fmt::Display'
/*
impl fn fmt(&self, f: &mut fmt::Formatter) -> Result {
            write!(f,"{}", self.x) //There is no ';' here
}
*/
//There is no ';' in the last 'write!' since that is what is asined to 'Result'