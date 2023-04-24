//loops

//Rust has an infine loop keyword, kind of like 'while True' in other languages
//Also there is 'break' that can break the loop at any time

//Lastly there is continue that allows you to skip an iteration and carry on to the
//next one 

fn main() {
    let mut count = 0_u32;

    println!("Let's count until infinity!");

    //Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("Three");

            //Skip the rest of this iteration
            continue;
        } else if count == 42 {
            println!("The answer to life, don't know the question tho...");

            continue;
        }
        println!("{}", count);

        if count == 50 {
            println!("Ok, that enough");
            
            //Exit the loop
            break
        }
    }
}