use std::fmt::{self, Display, Formatter};

//I will only put the excersice here since the example is redundant
#[derive(Debug)]

struct Color{
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        //Write RGB
        write!(f, "RED ")?;
        //Write color as (a,b,c)
        write!(f, "({}, {}, {})", self.red, self.green, self.blue)?;
        //Write color as HEX
        write!(f, " {}{:02X}{:02X}{:02X}", "0x", self.red, self.green, self.blue)
    }
}

fn main() {
    let my_color: Color = Color { red: (0), green: (13), blue: (255) };
    println!("{}",my_color);
}