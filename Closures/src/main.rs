//Closures are like quick functions that don't need a name. Sometimes they are called lambdas. 
//Closures are easy to find because they use || instead of (). 
//They are very common in Rust, and once you learn to use them you will wonder how you lived without them.

//You can bind a closure to a variable, and then it looks exactly like a function when you use it:
// fn main() {
//     let my_closure =  || println!("This is a closure");
//     my_closure();

// }
// -----------------------------------------------------------------------------
// So this closure takes nothing: || and prints a message: This is a closure.

// In between the || we can add input variables and types, like inside () for a function

// fn main() {
//     let my_closure =  |x: i32| println!("{}", x);
//     my_closure(5);
//     my_closure(5+5);

// }
// -----------------------------------------------------------------------------------------------

// Here is another example: the unwrap_or method that we know that you can use to give a value if unwrap doesn't work. 
// Before, we wrote: let fourth = my_vec.get(3).unwrap_or(&0);. 
// But there is also an unwrap_or_else method that has a closure inside. 
// So you can do this:

fn main() {
    let my_vec = vec![8, 9, 10];

    let fourth = my_vec.get(3).unwrap_or_else(|| { // try to unwrap. If it doesn't work,
        if my_vec.get(0).is_some() {               // see if my_vec has something at index [0]
            &my_vec[0]                             // Give the number at index 0 if there is something
        } else {
            &0 // otherwise give a &0
        }
    });

    println!("{}", fourth);
}

