use std::mem; //You can use this library to get the amount of memory
//alocated for a value, in this case arrays are stack alocated

//This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("First element: {}", slice[0]);
    println!("This slice has {} elements", slice.len());
}

fn main() {
    //Fixes size array (type signature is superfluos)
    let xs: [i32; 5]  = [1, 2, 3, 4, 5];

    //All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    //You can print values of an array with the index just like in python
    println!("First element: {}", xs[0]);
    println!("Second element: {}", xs[1]);
    
    //You can use len() to get the lenght of the array
    println!("Lenght of array: {}", xs.len());

    // Arrays are stack alocated?
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be borrowed as slices
    println!("Borrow the WHOLE array as a slice:");
    analyze_slice(&xs);

    //Slices can point to a section of an array
    //They are of the form [s_index .. end_index].
    //s_index is the first position of the slice
    //end_index is n+1 of the last posicion of the slice
    println!("Borrrow a section fo the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    //Example of empty slice '&[]'
    let empty_array: [u32; 0] = []; //Define empty array
    assert_eq!(&empty_array, &[]); //Create empty slice of that array
    assert_eq!(&empty_array, &[][..]); //Same but more verbose

    //Arrays can be safely accessed using '.get', which returns an
    //'Option'. This can be matched as shown below, or used with
    //'.expect()' if you would like th eprogram to exit with a nice
    //message instead of hapily continue
    for i in 0..xs.len() + 1 { //Create a slice of the whole array and iterate i trough it.
        match  xs.get(i) { //Try to get the value in xs using its match array.get(index)
            Some(xval) => println!("{}: {}", i, xval), //If the value exists it prints the index and the value
            None => println!("Slow down! {} is too far!", i), //If no value exists it prints the last index that is outside the array and stops
        }
    }

    //Trying to index out of bounds gives an error
    //println!("{}", xs[5]);
}