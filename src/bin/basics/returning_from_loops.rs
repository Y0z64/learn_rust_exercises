//Returning from loops
//If a loop returns a value you might need to pass it to the rest of the code, put it
//after the break and it will be returned by the loop expression
fn main() {
    let mut counter = 0;

    let result = loop { //This will assign whatever is returned by the loop to 'result'
        counter += 1;

        if counter == 10 { 
            break counter * 2; //This counter * 2 (in this case 20) will be returned by the loop
        }
    };

    assert_eq!(result, 20);
}