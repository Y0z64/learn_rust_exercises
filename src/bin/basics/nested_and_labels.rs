//You can put labels in loops, with sintax 'label
//You can call those labels in nested loops to 'break', 'continue' them
#![allow(unreachable_code, unused_labels)]

fn main() {
    'outer: loop { //<- the 'label is 'outher
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            //This would break only the inner loop
            break;

            //This breaks the outher loop
            //break 'outer;
        }
        println!("This point will never be reached");
        break;
    }
    println!("Exited the outer loop");
}