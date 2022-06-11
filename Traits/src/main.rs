//A trait in Rust is a group of methods that are defined for a particular type. 
//Traits are an abstract definition of shared behavior amongst different types. 
//So, in a way, traits are to Rust what interfaces are to Java or abstract classes are to C++. 
//A trait method is able to access other methods within that trait.

//let's look at how to make a trait. 
//The important thing to remember about traits is that they are about behaviour. 
//To make a trait, write trait and then create some functions.

struct Animal { // A simple struct - an Animal only has a name
    name: String,
}

trait Dog { // The dog trait gives some functionality
    fn bark(&self) { // It can bark
        println!("Woof woof!");
    }
    fn run(&self) { // and it can run
        println!("The dog is running!");
    }
}

impl Dog for Animal {
    fn run(&self) {
        println!("{} is running!", self.name);
    }
}

fn main() {
    let rover = Animal {
        name: "Rover".to_string(),
    };

    rover.bark(); // Now Animal can use bark()
    rover.run();  // and it can use run()
}

