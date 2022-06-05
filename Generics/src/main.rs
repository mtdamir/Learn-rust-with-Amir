//  generics = can be one type, can be another type

// We use generics to create definitions for items like function signatures or structs, 
// which we can then use with many different concrete data types. 
// -----------------------------------------------------------------------------------------

// In functions, you write what type to take as input:

// fn return_number(numner: i32) -> i32 {
//     println!("here is your number.");
//     numner
// }

// fn main() {
//     let number = return_number(5);
// }

// But what if you want to take more than just i32? 
// You can use generics for this. 
// Generics means "maybe one type, maybe another type".
// ----------------------------------------------------------------
// For generics, you use angle brackets with the type inside, like this: <T> This means "any type you put into the function". 
// Usually, generics uses types with one capital letter (T, U, V, etc.), 
// though you don't have to just use one letter

// fn return_thing<T>(thing: T) -> T {
//     println!("here is your thing.");
//     thing
// }

// fn main() {
//     let my_string = return_thing(String::from("I'm String"));
//     println!("{}", my_string);
// }
// The important part is the <T> after the function name. 
// Without this, Rust will think that T is a concrete (concrete = not generic) type, like String or i8

// ------------------------------------------------------------------------------------------------------------
// You will remember that some types in Rust are Copy, some are Clone, some are Display, some are Debug, and so on. 
// With Debug, we can print with {:?}. So now you can see that we have a problem if we want to print T

// use std::fmt::Debug; // Debug is located at std::fmt::Debug. So now we can just write 'Debug'.

// fn print_number<T: Debug>(number: T) { // <T: Debug> is the important part
//     println!("Here is your number: {:?}", number);
// }

// fn main() {
//     print_number(5);
//     print_number(5.75);
// }
// ---------------------------------------------------------------------------------------------------------------

use std::fmt::{Display, Debug};
// This used to compare like greater than less than
use std::cmp::PartialOrd;

#[derive(Debug)]
struct Animal {
    name: String,
    age: u8 
}

fn compare_and_display<T:Display, U:Display + PartialOrd, MyGenericType: Debug>(statement: T, num_1: U, num_2: U, animal: MyGenericType){
    println!("{}! is {} greater than {}? {}",
    statement,num_1,num_2 , num_1 > num_2
    );
    println!("By the way, I have an animal: {:?}", animal);
}

fn main() {
    let cat = Animal {
        name: "Charlie".to_string(),
        age: 2
    };

    compare_and_display("Listen up!", 9, 8, cat);
}


