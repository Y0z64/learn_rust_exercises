//Enums function like lists of types for objects that might
//Each name and type is independent and diferent
//For example:

enum WebEvent {
    //An 'enum' variant may either be 'unit-like'
    PageLoad,
    PageUnLoad,
    //tuples
    KeyPress(char), //you can keep adding names in here
    Paste(String),
    //or c-like
    Click { x: i64, y: i64},
}

//Exmaple 2 enum
enum VeryLongNameForADumbAsFuckEnum {
    Add,
    #[allow(dead_code)]
    Substract,
}

//You can reference types with type aliases, for example
type Operations = VeryLongNameForADumbAsFuckEnum; //This creates a type alias

//The most common way to use type alias is with 'self'
impl VeryLongNameForADumbAsFuckEnum {
    #[allow(dead_code)]
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Substract => x - y,
        }
    }
}



//A function which takes a 'WebEvent' enum as an argument
//and returns nothing
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnLoad => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s), //You can print "" quotation marks if next to /"Hello/" 
        WebEvent::Click { x, y } => {
            println!("clciked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    //'to_owned()' creates an owned 'String' from a string slice
    //so basically creates a copy of a type from a reference or other type
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click {x: 20, y:80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnLoad;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    //Exercise 2
    #[allow(unused_variables)]
    let x = Operations::Add; //Definition using type alias
}
