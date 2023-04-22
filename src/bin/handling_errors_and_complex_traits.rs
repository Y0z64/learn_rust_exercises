/*
You can generate kind of complex fmt::Display traits, for example:
for values that need to be iterated upon to be correctly displayed.
One of those values is vec<T>
*/
//For example

use std::fmt; //Import fmt

//Define structure named 'List' that contains a 'Vec'
struct List(Vec<i32>); //This creates a 'List' that is actually just a vector or an array that has not specific number for values, kind of like Python arrays

impl fmt::Display for List{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //Extract the value using tuple indexing
        let vec = &self.0; //This will extract the first value of the vector

        write!(f, "[")?;

        //Iterate and enumerate over v
        for (count, v) in vec.iter().enumerate() {
            //Add a comma or each element except the first
            if count != 0 {
                write!(f, ", ")?; 
                } //? is for handling errors
            write!(f, "{}: {}",count, v)?; //The "{}: " was added after to comply with the instructions of the exercise
            }
            //Close and resturn the fmt::Result
            write!(f, "]") //The final write in a Display impl must not have a ; to be able to return fmt::Return correctly
    }
}

/*
The compiler will usually tell you when there is need for a ? operator
since Rust handles errors almost automatically.
The point of all of this exercise is to learn that all posible outcomes must be
handled even if they are just errors */

fn main() {
    let numbers: List = List(vec![1, 2, 3]);
    println!("{}", numbers);
}