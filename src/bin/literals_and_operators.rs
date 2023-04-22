
fn main() {
    //Integrer adition
    println!("\n1 + 2 = {}", 1u32 + 2);

    //Scientific notation
    println!("\n1e4 is {}", 1e4);
    println!("-2.5e-7 is {}", -2.5e-7);

    //Boolean logic
    println!("\ntrue AND false: {}", true && false);
    println!("true OR false: {}", true || false);
    println!("NOT true: {}", !true);

    // Bitwise operations
    println!("\n0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    //I dont even know what this is ^

    // Use underscores to improve readability!
    println!("\nOne million is written as {}", 1_000_000u32); //Use underscores in numbers for readability: 1_000 = 1000 AND 0.000001 = 0.000_001

    //Express ints as hex, oct, or bin
    println!("\nhex: {:0x} or Hex: {:0X}", 255i32, 255i32);
    println!("oct: {:0o}", 255i32);
    println!("bin: {:0b}", 255i32);

}