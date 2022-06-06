//We understand enums and generics now, so we can understand Option and Result. 
//Rust uses these two enums to make code safer.

//Option means = maybe there's something, maybe  there's not

//We will start with Option.

fn take_fift(value: Vec<i32>) -> Option<i32>{
    if value.len() < 5 { // .len() gives the length of the vec.
                        // It must be at least 5.
        None
    } else {
        Some(value[4])
    }
}

fn main() {
    let new_vec = vec![1,2];
    let bigger_vec =  vec![1,2,3,4,5];
    println!("{:?}, {:?}", take_fift(new_vec), take_fift(bigger_vec).unwrap());
}

// We can get the value inside an option with .
// unwrap(), but be careful with .unwrap(). 
// It's just like unwrapping a present: maybe there's something good inside, or maybe there's an angry snake inside. 
// You only want to .unwrap() if you are sure. 
// If you unwrap a value that is None, the program will panic


