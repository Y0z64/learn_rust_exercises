#![allow(unused_variables)]
#![allow(unused_assignments)]
fn main() {
    //Type anotation
    let logical: bool = true; //Regular annotation
    
    let something = 5i32; //Suffix annotation, im never using this
    
    //Rust can also use defaults
    let defulat_float = 3.0; //f64
    let default_integrer = 5; //i32

    let mut inferred_type = 12; //Types can be inferred
    inferred_type = 81927381273i64; //which means you can set the type in other places than the initialization
    //Here it is put via a sufix ^

    //mutable variables can be changed, a var is made mutable with mut
    let mut mutable = 12;
    mutable = 14;

    // Error! the type of a variable can't be changed.
    // mutable = true;

    // but they can be overwritten complitelly
    let mutable = true;
}