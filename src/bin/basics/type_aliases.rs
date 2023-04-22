// The statement 'type' allows you to give a new name
// to an existing type

type NanoSecond = u64;
type Inch = u64;
type U64 = u64;
//Remember to use UpperCamelCase (PascalCase) or the compiler will complain

fn main() {
    //'NanoSecond' = 'Inch' = 'U64' = 'u64'
    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    //Note that type aliases don't provide any extra type safety, because aliases
    //are not *not* new types
    println!("{} nanoseconds + {} inches = {} unit?",
            nanoseconds,
            inches,
            nanoseconds + inches);
}
