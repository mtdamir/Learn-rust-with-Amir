//A trait in Rust is a group of methods that are defined for a particular type. 
//Traits are an abstract definition of shared behavior amongst different types. 
//So, in a way, traits are to Rust what interfaces are to Java or abstract classes are to C++. 
//A trait method is able to access other methods within that trait.

//let's look at how to make a trait. 
//The important thing to remember about traits is that they are about behaviour. 
//To make a trait, write trait and then create some functions.

// struct Animal {
//     name: String,
// }

// trait Dog {
//     fn bark(&self); // bark() says it needs a &self and returns nothing
//     fn run(&self); // run() says it needs a &self and returns nothing.
//                    // So now we have to write them ourselves.
// }

// impl Dog for Animal {
//     fn bark(&self) {
//         println!("{}, stop barking!!", self.name);
//     }
//     fn run(&self) {
//         println!("{} is running!", self.name);
//     }
// }

// fn main() {
//     let rover = Animal {
//         name: "Rover".to_string(),
//     };

//     rover.bark();
//     rover.run();
// }
// reading and understanding trait implementation

// #[derive(Clone, Copy)]

// struct Numbers {
//     number1: u8,
//     number2: u8,
// }

// fn take_number(number: Numbers){

// }

// fn main() {
//     let my_number = Numbers {
//         number1: 8,
//         number2: 12,
//     };

//     take_number(my_number);
//     take_number(my_number);
// }
// --------------------------------------------------------------------------------------------

//So when you create a trait, you must think: "Which functions should I write? And which functions should the user write?" 
//If you think the user should use the function the same way every time, then write out the function. 
//If you think the user will use it differently, then just write the function signature.

//So let's try implementing the Display trait for our struct. First we will make a simple struct:
// use std::fmt;

// struct Cat {
//     name: String,
//     age: u8,
// }

// impl fmt::Display for Cat {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//       let cat_type = match self.age {
//           0..=2 => "kitten",
//           3..=4 => "adult",
//           _ => "old"
//       };

//       write!(f, "{} is a cat who is {} years old. and there for {}", self.name, self.age, cat_type)
//   }
// }

// fn main() {
//     let mr_mantle = Cat {
//         name: "Reggie Mantle".to_string(),
//         age: 4,
//     };

//     println!("{}", mr_mantle);
// }
// ----------------------------------------------------------------------
//Let's look at another example that is even more connected to just behaviour. 
//We'll imagine a fantasy game with some simple characters. 
//One is a Monster, the other two are Wizard and Ranger. 
//The Monster just has health so we can attack it, the other two don't have anything yet. But we made two traits. 
//One is called FightClose, and lets you fight up close. 
//The other is FightFromDistance, and lets you fight from far away. 
//Only Ranger can use FightFromDistance. Here's what it looks like


use std::fmt::Debug;

struct Monster{
    health: i32,
}

#[derive(Debug)]
struct Wizard {
    health: i32,
}

#[derive(Debug)]
struct Ranger {
    health: i32,
}

trait Magic {}
trait FightClose {}
trait FightFromDistance{}

impl FightClose for Ranger{}
impl FightClose for Wizard{}
impl FightFromDistance for Ranger{}
impl Magic for Wizard{}

fn attack_with_bow<T: FightFromDistance + Debug>(character: &T, opponent: &mut Monster, distance: u32) {
    if distance < 10 {
        opponent.health -= 10;
        println!(
            "You attack with your bow. Your opponent now has {} health left.  You are now at: {:?}",
            opponent.health, character
        );
    }
}

fn attack_with_sword<T: FightClose + Debug>(character: &T, opponent: &mut Monster) {
    opponent.health -= 10;
    println!(
        "You attack with your sword. Your opponent now has {} health left. You are now at: {:?}",
        opponent.health, character
    );
}

fn fire_ball<T:Magic + Debug>(character: &T, opponent: &mut Monster, distance: u32) {
    if distance < 15 {
        opponent.health -= 20;
        println!("You raise your hands and cast a fireball! Your opponent now has {} health left. You are now at: {:?}",
    opponent.health, character);
    }
}

fn main() {
    let radagast = Wizard { health: 60 };
    let aragorn = Ranger { health: 80};

    let mut uruk_hai = Monster { health: 40};

    attack_with_sword(&radagast, &mut uruk_hai);
    attack_with_bow(&aragorn, &mut uruk_hai, 8);
    fire_ball(&radagast, &mut uruk_hai, 8)
}

