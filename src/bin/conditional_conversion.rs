//Similar to From and Into
//TryFrom and TryInto are faliable conversions for when conversion is not posible
//100% of the times but still needs to be implemented

//Another important diferente is that these return 'Results'

use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
} //This implementation for example is a conditional conversion that depends on a condition to be true to be able to convert

fn main(){
    //TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    //TryInto
    let result: Result<EvenNumber, ()> = 8_i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5_i32.try_into();
    assert_eq!(result, Err(()));
}
