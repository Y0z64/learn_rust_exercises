//This is how you import modules
use std::fmt;

//This is how you define custom structures or "types"
struct Structure(i32);

//This is how you implement 'fmt::Display' to a custom
//type for correct printing formating
impl fmt::Display for Structure {
    //This traid requires 'fmt; with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

/*For generic cases like in the case of Vec<T> there is no fmt::Display trait
implemented for default since the type is ambiguos. The guide recomends using only
fmt::Debug for those cases, but that does not include cutom types only generic ones */

//Second exameple on how to implement
#[derive(Debug)] //This is added for comparision only
struct MinMax(i64, i64);

//Display '{}' trait implementation for MinMax
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //use 'self.n' to refer to each positional data point
        write!(f, "({}, {})", self.0, self.1)
    }
}

//For example 2
//We can also define named arguments when creating a custom type
/* I had to learn the hard way that Rust does not allow names that start
with nubers, but it does with names that start with underscores '_' */
#[derive(Debug)]
struct Vector2D {
    //You can use f32 or f64 for floats, in this case im using ints
    x: i64,
    y: i64,
}

//Implement Display '{}' trait for Point2D
impl fmt::Display for Vector2D { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

/*Exercise */
//Define Complex
#[derive(Debug)]
struct Complex{
    real: f64,
    imag: f64,
}

//Implement fmt::Display
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    //Defining a variable with the custom type MinMax
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Debug: {:?}", minmax);
    println!("Display: {}\n", minmax);

    //For types with specified keys you have to define with {} and not (),
    //and specify wich 'key: value' for each key
    let my_vector: Vector2D = Vector2D{x: 0, y: 28};

    println!("Compare structures:");
    println!("Debug: {:?}", my_vector);
    println!("Display: {}\n", my_vector);

    /*
    Here you already implemented the traits for Debug '{;?}' and Display 
    '{}'. Still you still have to implement the traits for other formats,
    for example {:b} for binary
    */
    
    //define var with custom type Complex
    let my_complex: Complex = Complex{
        real: 3.3,
        imag: 7.2,
    };

    println!("Compare structures for exercise:");
    println!("Display: {}", my_complex);
    println!("Debug: {:?}", my_complex);
}