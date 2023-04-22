//Variable bindings or typing
//You come from typescript so you already know this

fn main() {
    let an_int = 1u32; //sufix
    let a_boolean = true; //no need for typing
    let unit = (); //empty tuple

    //copy 'an_integrer' into 'copied_integrer'
    let copied_integrer = an_int; //Custom types that not have the Copy or Clone trait implemented will not be able to be copied

    println!("An integrer {:?}", copied_integrer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    //The compiler warns about unused variable bidings; these warnings can
    //be silenced by prefixing the variable name with an underscore
    let _unused_var = 3_u32;

    #[allow(unused_variables)]
    let noisy_unused_var = 2_u32;
    // ^ Prefix with '_' to shut warnings
}