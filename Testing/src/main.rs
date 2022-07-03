//Testing is a good subject to learn now that we understand modules. 
//Testing your code is very easy in Rust, because you can write tests right next to your code.

//The easiest way to start testing is to add #[test] above a function. Here is a simple one:

// #[test]
// fn two_is_two() {
//     //Let's change assert_eq!(2, 2) to assert_eq!(2, 3) and see what we get. 
//     //When a test fails you get a lot more information:

//     // assert_eq!(2, 2);
//     assert_eq!(2, 3);

//     //assert_eq!(left, right) is the main way to test a function in Rust. 
//     // If it doesn't work, it will show the different values: left has 2, but right has 3.
// }

// ---------------------------------------------------------------------------------------------------------------
fn main(){

}

fn return_two() -> i8 {
    2
}

fn return_six() -> i8 {
    4 + return_two()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_six() {
        assert_eq!(return_six(),6)
    }

    #[test]
    fn it_returns_two() {
        assert_eq!(return_two(), 2)
    }
}