//String conversion

//Converting to a String is as simple as implementing 'ToString' trait for the type
//Rather than doing so directly, you SHOULD implement 'fmt::Display' trait which
//will provide 'ToString' and also allows it to be printed
use std::fmt;

struct Circle{
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle: Circle = Circle { radius: (5) };
    println!("{}", circle.to_string());
//                       ^^^^^ This is not necesary since the 'fmt::Display' trait
//Already allows displaying using '{}', but still it shows that the .to_string()
//method is avalible since 'fmt::Display' also adds 'ToString'

//---------Parsing
//This is the way to conver a string into a number
//The aproach to to this is using 'parse', either arraing for type inference (specifing the type)
//or using 'turbofish' sintax

//This will convert the string into the type specified as long as the 'FromStr'
//trait is implemented for that type.
//I did not do that since I actually dont know exactly how, but lets see an example
//that was already implemented in the standard library
    let parsed: i32 = "5".parse().unwrap(); //Type inference
    let turbo_parsed = "10".parse::<i32>().unwrap(); //Turbofish sintax
    //                      ^^^^^^^^^^^^^^^^^^^this is equal to 'parse()' but here we call the 'parse' method implemented in the type instead of letting the compiler do it with type inference
    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}