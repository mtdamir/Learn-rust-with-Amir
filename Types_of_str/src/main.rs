// There is more than one type of &str. We have:

// String literals: you make these when you write let my_str = "I am a &str". 
// They last for the whole program, because they are written directly into the binary. 
// They have the type &'static str. ' means its lifetime, and string literal have a lifetime called static.

// Borrowed str: This is the regular &str form without a static lifetime. 
// If you create a String and get a reference to it, Rust will convert it to a &str when you need it. 
// For example:

fn prints_str(my_str: &str) { // it can use &String like a &str
    println!("{}", my_str);
}

fn main() {
    let my_string = String::from("I am a string");
    prints_str(&my_string); // we give prints_str a &String
}

