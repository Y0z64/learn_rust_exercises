// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.

//create structure and add the std::Debug
#[derive(Debug)]
struct Structura(i32);

//structure for second example
#[derive(Debug)]
#[allow(dead_code)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    //This will allow the custom structure to be displayed but is obviously not very
    //elegant
    println!("Debug print of custom structure: {:?}", Structura(3));

    //There is also pretty printing with {:#?}


    let name = "Yair";
    let age = 18;
    let yair = Person { name, age }; 
    println!("Normal debug printing {:?}", yair);
    println!("Pretty print of custom structure: {:#?}", yair);
}