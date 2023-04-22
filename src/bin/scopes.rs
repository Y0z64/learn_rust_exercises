//Scopes and shadowing

//Very easy to understand
// Brackets '{}' define blocks, and inside blocks there is scopes
//Inside the scopes you can define vars, cons, etc. But they wont be valid
//outside it unless specified (this is whats called lifespan)

//If there is a variable with a value in an outer scope, you are allowed to specify
//another value for that variable inside the inner scope, when the inner scope closes
//you will be able to use the value asigned in the outer scope

fn main() {
    //This binding lives in the main function
    let long_lived_binding = 1;

    //This is a block
    {
        //This binding is only valid inside this block
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }
    //End of block

    //The variable inside of the block does not exist in this scope
    //println!("outer short {}", short_lived_binding); //-> This will give an Error!

    println!("outer long: {}", long_lived_binding);   

    //Example 2
    let my_var = 12;
    println!("This is in the outer scope: {}", my_var);
    {
        let my_var = 24;
        println!("This is inside scope: {}", my_var);
    }
    println!("This is outer scope as well: {}", my_var);

    //Using let again also *shadows* the value of a variable but it actually just
    //reasigns it a value
    let my_var = 12083;
    println!("This is a shadowed var: {}", my_var);
}

