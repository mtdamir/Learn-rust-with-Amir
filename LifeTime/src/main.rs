//A lifetime means "how long the variable lives". 
//You only need to think about lifetimes with references. 
//This is because references can't live longer than the object they come from

// fn returns_reference() -> &str {
//     let my_string = String::from("I am a string");
//     &my_string // ⚠️
// }

// fn main() {}

// The problem is that my_string only lives inside returns_reference. 
// We try to return &my_string, but &my_string can't exist without my_string. So the compiler says no

//missing lifetime specifier means that we need to add a ' with the lifetime. Then it says that it contains a borrowed value, but there is no value for it to be borrowed from. That means that I am a str isn't borrowed from anything. 
//It says consider using the 'static lifetime by writing &'static str. 
//So it thinks we should try saying that this is a string literal.

fn returns_str() -> &'static str  {
    let my_string = String::from("I'm String");
    "I'm String"
}

fn main(){
    let my_str = returns_str();
    println!("{}", my_str);
}

// So now fn returns_str() -> &'static str tells Rust: "don't worry, we will only return a string literal". 
// String literals live for the whole program, so Rust is happy. You'll notice that this is similar to generics. 
// When we tell the compiler something like <T: Display>, we promise that we will only use inputs with Display. 
// Lifetimes are similar: we are not changing any variable lifetimes. 
// We are just telling the compiler what the lifetimes of the inputs will be.