/* There are three types of structs
-Tuple structs, which are, basically, named tuples.
-The classic C structs
-Unit structs, which are field-less, are useful for generics.
 */
#![allow(dead_code)] //using #! you can apply this "decorators" to the whole code
use std::fmt::{self, Display, Formatter};

//A person struct
#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

//A unit struct
struct Unit;

//A tuple struct
struct Pair(i32, f32);

//A struct with two fields
#[derive(Clone, Copy, Debug)]
struct Point {
    x: f32,
    y: f32,
}

//Structs can be reused as fields fo another struct
#[derive(Debug)]
struct Rectangle {
    //Specified from the top left and bottom right corners in space
    top_left: Point,
    bottom_right: Point,
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let Rectangle {
            top_left: Point { x: point1_x, y: point1_y },
            bottom_right: Point { x: point2_x, y: point2_y },
        } = self;
        write!(f, "Figure: ({}, {}) to ({}, {})",
            point1_x,point1_y,point2_x,point2_y)
    }
}

fn rect_area(rect: Rectangle) -> f64{
    let Rectangle {
        top_left: Point { x: top_left_x, y: top_left_y},
        bottom_right: Point { x: bottom_left_x, y: bottom_left_y },
    } = rect;
    //YOU CAN DESTRUCT NESTED TYPES WITH NESTED DESTRUCTION TO ACCESS DEEP NESTED VALUES DIRECTLY, THAT IS METAL AS FUCK NGL
    let height = top_left_y - bottom_left_y;
    let width = top_left_x - bottom_left_x;

    let area = height as f64 * width as f64;
    return area.abs();
}

fn square(point: Point, value: f32) -> Rectangle {
    let new_point = Point {
        x: point.x + value,
        y: point.y + value,
    };
    let rect: Rectangle = Rectangle { top_left: point, bottom_right: (new_point) };
    return rect;
} 

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter"); //This is the best way to create strings in Rust, specially if they need to be modified
    let age = 27;
    let peter = Person { name, age };

    //Print debug struct 
    println!("{:?}", peter);

    // Instantiate a 'Point;
    let my_point: Point = Point {x: 10.3, y:0.4};

    //Acess the fields of the point
    println!("point coordinates: ({}, {})", my_point.x, my_point.y);
    //THIS IS THE COMMON AND EASY WAY TO DO IT

    //Make a new point by using a struct update syntax to use the fields of our
    //other one
    let bottom_right = Point { x:5.2, ..my_point }; //using slicing to reference the other defined point

    //'bottom_right.y' will be the same as 'my_point.y' beacuse we used that field from point
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);
    //YOU CAN ALSO BORROW VALUES FROM ANOTHER TOUPLES USING SLICING

    //Destructure the point using a 'let' binding
    let Point {x : left_edge, y: top_edge } = my_point;

    let my_rectangle = Rectangle {
        // struct initialization is an expresion too
        top_left: Point { x: left_edge, y: top_edge},
        bottom_right: bottom_right,
    };
    //YOU CAN ALSO DESTRUCT A TYPE IN ITS DEFINITION TO USE ITS VALUES EXTERNALLY
    println!("Rectangle: {}", my_rectangle);

    //Instantiate a unit struct
    let _unit = Unit;

    //Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    //YOU CAN ACCESS TOUPLE VALUES EVEN IF THEY DONT HAVE A KEY DEFINED
    // Destructure a tuple struct

    let Pair(integrer, decimal) = pair;
    //YOU CAN ALSO DESTRUCT WITH NO KEY DEFINED, ONLY USING THE ORDER OF THE ARGUMENTS
    println!("pair contanins {:?} and {:?}", integrer, decimal);

    let point_1: Point = Point { x: (1.0), y: (-3.0) };
    let point_2: Point = Point { x: (4.0), y: (0.0) };
    let my_rect: Rectangle = Rectangle { top_left: (point_1), bottom_right: (point_2) };
    println!("Area is equal to {}", rect_area(my_rect));

    let my_square: Rectangle = square(point_1, 3.0);
    println!("Square: {}", my_square);
}