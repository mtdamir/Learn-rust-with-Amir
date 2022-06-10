//There is an even shorter way to deal with Result (and Option), shorter than match and even shorter than if let. 
//It is called the "question mark operator", and is just ?. 
//After a function that returns a result, you can add ?. This will:

//return what is inside the Result if it is Ok
//pass the error back if it is Err

//We can try this with .parse() again. 
//We will write a function called parse_str that tries to turn a &str into a i32. It looks like this

// use std::num::ParseIntError;

// fn parse_str(input: &str) -> Result<i32, ParseIntError> {
//     let parsed_number = input.parse::<i32>()?; // Here is the question mark
//     Ok(parsed_number)
// }

// fn main() {}
// ------------------------------------------------------------------------------------------
// fn parse_str(input: &str) -> Result<i32, std::num::ParseIntError> {
//     let parsed_number = input.parse::<i32>()?;
//     Ok(parsed_number)
// }

// fn main() {
//     let str_vec = vec!["Seven", "8", "9.0", "nice", "6060"];
//     for item in str_vec {
//         let parsed = parse_str(item);
//         println!("{:?}", parsed);
//     }
// }
// -------------------------------------------------------------------------------

// PANIC

// Rust has a panic! macro that you can use to make it panic. 
// It is easy to use:


// fn main() {
//     panic!("Time to panic!");
// }

//The message "Time to panic!" displays when you run the program: thread 'main' panicked at 'Time to panic!'
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        // Division by zero triggers a panic
        panic!("division by zero");
    } else {
        dividend / divisor
    }
}

fn main () {
    let _x =  Box::new(0i32);

    division(3,0);

    println!("This point won't be reashrd! ")
}
