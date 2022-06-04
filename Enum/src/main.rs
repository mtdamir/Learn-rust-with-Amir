// Enum can hold data
// ------------------------

// // create the enum with two choices
// enum ThingInTheSky {
//     Sun,
//     Stars,
// }

// // With this function we can use an i32 to create ThingsInTheSky
// fn create_skystate(time: i32) -> ThingInTheSky {
//     match time {
//         6..=18 => ThingInTheSky::Sun, // Between 6 and 18 hours we can see the sun
//         _ => ThingInTheSky::Stars, // Otherwise, we can see stars
//     }
// }

// // With this function we can match against the two choices in ThingsInTheSky.
// fn check_skystate(state: &ThingInTheSky){
//     match state {
//         ThingInTheSky::Sun => println!("I can see the sun!"),
//         ThingInTheSky::Stars => println!("I can see the stars!")
//     }
// }

// fn main() {
//     let time =19;
//     let skystate = create_skystate(time); // create_skystate returns a ThingsInTheSky
//     check_skystate(&skystate); // Give it a reference so it can read the variable skystate
// }
// -----------------------------------------------------------------------------------------------------
//  And now we put data in Enum

// enum ThingsInTheSky {
//     Sun(String), // Now each variant has a string
//     Stars(String),
// }

// fn create_skystate(time: i32) -> ThingsInTheSky {
//     match time {
//         6..=18 => ThingsInTheSky::Sun(String::from("I can see the sun!")), // Write the strings here
//         _ => ThingsInTheSky::Stars(String::from("I can see the stars!")),
//     }
// }

// fn check_skystate(state: &ThingsInTheSky) {
//     match state {
//         ThingsInTheSky::Sun(description) => println!("{}", description), // Give the string the name description so we can use it
//         ThingsInTheSky::Stars(n) => println!("{}", n), // Or you can name it n. Or anything else - it doesn't matter
//     }
// }

// fn main() {
//     let time = 8; // it's 8 o'clock
//     let skystate = create_skystate(time); // create_skystate returns a ThingsInTheSky
//     check_skystate(&skystate); // Give it a reference so it can read the variable skystate
// }
// ------------------------------------------------------------------------------------------------------

enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry, 
}

fn match_mood(mood: &Mood) -> i32 {
    use Mood::*;
    let happiness_level = match mood {
        Mood::Happy => 10, // Here we type Mood:: every time
        Mood::Sleepy => 6,
        Mood::NotBad => 7,
        Mood::Angry => 2,
    };

    happiness_level
}

fn main(){
    let my_mood = Mood::Angry;
    let happiness_level = match_mood(&my_mood);
    println!("Out of 1 to 10, my happiness is {}", happiness_level);
}
