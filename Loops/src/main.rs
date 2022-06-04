//With loops you can tell Rust to continue something until you want it to stop.
//You use loop to start a loop that does not stop, unless you tell it when to break.
// fn main() {
//     let mut counter = 0;  // set a counter to 0
//     loop {
//         counter += 1; // increase the counter by 1
//         println!("The counter is now: {}", counter);
//         if counter == 5 {
//             break;
//         }
//     }
// }
// ---------------------------------------------------------------------------

//If you have a loop inside of a loop, you can give them names. 
//With names, you can tell Rust which loop to break out of. 
//Use ' (called a "tick") and a : to give it a name

// fn main() {

//     let mut counter = 0;
//     let mut counter2 = 0;
//     println!("Now entering the first loop");

//     'first_loop:loop {
//         counter += 1;
//         println!("The counter id {}",counter);
//         if counter  > 9 {
//             println!("Now starting the second loop");
//             'second_loop: loop {
//                 println!("the second counter is {}", counter2);
//                 counter2 += 1;
//                 if counter2  == 3{
//                     break 'first_loop;
//                 }
//             }
//         }
        
//     }
// }

// ------------------------------------------------------------------------------------

// While Loop

// fn main() {
//     let mut counter = 0;

//     while counter < 5 {
//         counter +=1;
//         println!("The counter is now: {}", counter);
//     }
// }
// ------------------------------------------------------------------

// A (for loop) lets you tell Rust what to do each time. 
// But in a (for loop), the loop stops after a certain number of times. 
// (for) loops use ranges very often. You use .. and ..= to create a range.

// .. creates an exclusive range: 0..3 creates 0, 1, 2.
// ..= creates an inclusive range: 0..=3 = 0, 1, 2, 3.

fn main() {
    for number in 0..3 {
        println!("The number is: {}", number);
    }

    for number in 0..=3 {
        println!("The next number is: {}", number);
    }
}
