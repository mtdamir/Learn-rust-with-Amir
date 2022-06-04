fn main() {
    // let  a = String::from("salam");
    // let b = a;
    // println!("a = {}", a);
    // println!("b = {}", b);
    // drop(a);
    // drop(b);

    let mut a  = String::from( "hello");
    a = some_function(a);
    println!("in main function = {}", a);
}

fn some_function(input:String) -> String{
    println!("some function: {}", input);
    input
}
