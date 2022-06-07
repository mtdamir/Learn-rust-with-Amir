//We understand enums and generics now, so we can understand Option and Result. 
//Rust uses these two enums to make code safer.

//Option means = maybe there's something, maybe  there's not

//We will start with Option.

// fn take_fift(value: Vec<i32>) -> Option<i32>{
//     if value.len() < 5 { // .len() gives the length of the vec.
//                         // It must be at least 5.
//         None
//     } else {
//         Some(value[4])
//     }
// }

// fn main() {
//     let new_vec = vec![1,2];
//     let bigger_vec =  vec![1,2,3,4,5];
//     println!("{:?}, {:?}", take_fift(new_vec), take_fift(bigger_vec).unwrap());
// }

// We can get the value inside an option with .
// unwrap(), but be careful with .unwrap(). 
// It's just like unwrapping a present: maybe there's something good inside, or maybe there's an angry snake inside. 
// You only want to .unwrap() if you are sure. 
// If you unwrap a value that is None, the program will panic
// --------------------------------------------------------------------------------------------------------------------
// Another Example

// The Option<T> enum has two variants:

// None: to indicate failure or lack of value, and
// Some(value): a tuple struct that wraps a value with type T.

// An integer division that doesn't `panic!`
// fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
//     if divisor == 0 {
//         // Failure is represented as the `None` variant
//         None
//     } else {
//         // Result is wrapped in a `Some` variant
//         Some(dividend / divisor)
//     }
// }

// // This function handles a division that may not succeed
// fn try_division(dividend: i32, divisor: i32) {
//     // `Option` values can be pattern matched, just like other enums
//     match checked_division(dividend, divisor) {
//         None => println!("{} / {} failed!", dividend, divisor),
//         Some(quotient) => {
//             println!("{} / {} = {}", dividend, divisor, quotient)
//         },
//     }
// }

// fn main() {
//     try_division(4, 2);
//     try_division(1, 0);

//     // Binding `None` to a variable needs to be type annotated
//     let none: Option<i32> = None;
//     let _equivalent_none = None::<i32>;

//     let optional_float = Some(0f32);

//     // Unwrapping a `Some` variant will extract the value wrapped.
//     println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

//     // Unwrapping a `None` variant will `panic!`
//     println!("{:?} unwraps to {:?}", none, none.unwrap());
// }
// ----------------------------------------------------------------------------------------------------
// RESULT
//Result is similar to Option, but here is the difference:

//Option is about Some or None (value or no value),
//Result is about Ok or Err (okay result, or error result).
//So Option is if you are thinking: "Maybe there will be something, and maybe there won't." 
//But Result is if you are thinking: "Maybe it will fail.

// enum Option<T> {
//     None,
//     Some(T),
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// fn main() {}

//So Result has a value inside of Ok, and a value inside of Err. 
//That is because errors usually contain information that describes the error.

//Result<T, E> means you need to think of what you want to return for Ok, 
//and what you want to return for Err

// fn give_result(input: i32) -> Result<(), ()> {
//     if input % 2 == 0 {
//         return Ok(())
//     } else {
//         return Err(())
//     }
// }

//Remember, the four methods to easily check are .is_some(), is_none(), is_ok(), and is_err().

// fn main() {
//     if give_result(5).is_ok() {
//         println!("It's Ok guys")
//     } else {
//         println!("It's error guys")
//     }

// }
// -----------------------------------------------------------------------------
// fn check_if_five(number: i32) -> Result<i32, String> {
//     match number {
//         5 => Ok(number),
//         _ => Err("Sorry, the number wasn't five.".to_string()), // This is our error message
//     }
// }

// fn main() {
//     let mut result_vec = Vec::new(); // Create a new vec for the results

//     for number in 2..7 {
//         result_vec.push(check_if_five(number)); // push each result into the vec
//     }

//     println!("{:?}", result_vec);
// }

// -------------------------------------------------------------------------------------
// Another Example

mod checked {
    // Mathematical "errors" we want to catch
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            // This operation would `fail`, instead let's return the reason of
            // the failure wrapped in `Err`
            Err(MathError::DivisionByZero)
        } else {
            // This operation is valid, return the result wrapped in `Ok`
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }
}

// `op(x, y)` === `sqrt(ln(x / y))`
fn op(x: f64, y: f64) -> f64 {
    // This is a three level match pyramid!
    match checked::div(x, y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match checked::sqrt(ln) {
                Err(why) => panic!("{:?}", why),
                Ok(sqrt) => sqrt,
            },
        },
    }
}

fn main() {
    // Will this fail?
    println!("{}", op(1.0, 10.0));
}






