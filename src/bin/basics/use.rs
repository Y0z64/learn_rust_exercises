/*Manual scoping refers to the use of C-like sintax like Lib::Pckg::Func::Var
We can use 'use' to make that easier*/

//Allow dead code since this file is an example
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    //Explictly 'use' each name so they are avalible without
    //manual scoping
    use crate::Status::{Poor, Rich}; //You can explicitly 'use' the values you need
    //Automatically 'use' each name inside 'Work'
    use crate::Work::*; //Or the whole pckg/lib with '*'

    //Equivalent to 'Status::Poor'
    let status = Poor;
    //Equivalent to 'Work::Civilian'
    let work = Civilian;

    match status {
        //Note the lack of scoping because of the explicit 'use' above/
        Rich => println!("The rich have lots of money"),
        Poor => println!("The poor are broke as hell..."),
    }

    match work {
        //Note again the lack of scoping
        Civilian => println!("The civilian is not in the army"),
        Soldier => println!("The soldiers is in the army"),
    }
}