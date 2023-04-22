use std::fmt::{self, Display, Formatter};
//Define structure
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);
 
 //implementation of Display 
 impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        //first row
        writeln!(f, "({} {})", self.0, self.1)?;
        //seccond row
        write!(f, "({} {})", self.2, self.3)
    }
}

fn transpose(a: Matrix) -> Matrix {
    Matrix(a.0, a.2, a.1, a.3)
}

fn main() {
    //You can define touples with specific types of any type and can be infinitelly long
    let my_tuple = (3, "hi", true);
    println!("{:?}", my_tuple); //You can print tuples with {:?}
    //Unless they are more than 12 values long

    let _long_tuple = (1,2,3,4,5,6,7,8,9,10,11,12,14);
    //println!("{:?}", _long_tuple); //<- This will not work

    //You can access tuple values by index using tuple indexing
    println!("{}", my_tuple.0);

    //You can make tuples of tuples
    let tuple_of_tuples = ((1,2,3),(4,5), 7);
    println!("{:?}", tuple_of_tuples);

    //You can create one element tuples but you need to add a comma in the parentesis
    //to diferentiate from literals inside a parenthesis
    println!("{:?}", (3,)); //This will create a tuple
    println!("{:?}", (2)); //This will not
    //You can also initialize single value tuples this way as variables

    //Tuples can be destructed, sort of like in python with the unpacking operator "*"
    let tuple_2unpack = (1, "hello", "world", false);

    let (a,b,c,d) = tuple_2unpack;
    println!("{:?}, {:?}, {:?}, {:?}", a,b,c,d);

    #[allow(unused_parens)]
    let my_matrix: Matrix = Matrix((1.1),(1.2),(2.1),(2.2));
    println!("{:?}", my_matrix);
    println!("{}", my_matrix);
    //Excercise
    println!("{}", transpose(my_matrix));
}
