//Mutability is also dependent on scope
//This is called freezing
fn main(){
    let mut _mutable_binding = 7_i32;

    {
        //Shadowing by immutable
        let _mutable_binding = _mutable_binding;
        //Since the variable is being reasigned a value inside this scope, and
        //the key word 'mut' is not after 'let', in this scope it is not mutable

        //_mutable_binding = 50; -> This will give a mistake
    }
    //
    _mutable_binding = 3;
    println!("Here the value is mutable: {}", _mutable_binding);
}