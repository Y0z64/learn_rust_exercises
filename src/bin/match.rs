fn main() {
    let number = 15;

    match number {
        1 => println!("Hello"),
        13..=19 => println!("Sex"), //Match an inclusive range
        _ => println!("Nope") //You can use _ for any other not managed option in match
    } //^ This is what is called a catch-all arm

    let boolean = true;
    //match is an expresion too, so it can be asigned as the value of a var
    let binary = match boolean {
        //The "arms" of a match must contain all posible values
        false => 0,
        true => 1,
        //^^^ If you comment out one of those options the match will break
    };

    println!("{} -> {}", boolean, binary);
}

/*You can make range with the .. operator, for example:
13..19 would be a range from 13 to 18 since the operator does not include the upper bond value
But you can modify the operator to ..= so it does match the upper value.
.. creates a range
..= creates an exclusive range */