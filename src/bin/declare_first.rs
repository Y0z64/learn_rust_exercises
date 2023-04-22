//It is posible to intialize variables and asign them a value later
//Just like C++

fn main(){
    let a_binding;

    {
        let x = 2;

        //initialize the binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);
    //This also allows to for the variable to be used outside the scope its value
    //was asigned into, like in this case

    let another_binding;

    //Error! Use of uninitialized binding
    //println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);

}