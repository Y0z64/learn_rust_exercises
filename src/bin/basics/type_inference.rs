//The type inference is pretyt smart
//It does not only look at the type of a variabl but also how it is used aftewards to
//infer its type

fn main() {
    //Because o the annotation, the compiler knows that 'elem' has type u8
    let elem = 5_u8;

    //Create an empty vector (a growable array (like the python ones))
    let mut vec = Vec::new();
    //At this point the compiler does not know the exact type of 'vec', it,
    //just knows that it's a vector of something ('Vec<_>')

    //Inset 'elem' in the vector
    vec.push(elem);
    //Now the compiler knows that 'vec' is a vector of 'u8's
    
    println!("{:?}", vec);
}