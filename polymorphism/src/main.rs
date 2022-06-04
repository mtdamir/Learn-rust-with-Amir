// trait Fly {
//     fn fly(&self);
//     fn land(&self);
// }

// struct Kaftar ();
// struct AirPlane();

// impl Fly for Kaftar {
//     fn fly(&self) {
//         println!("Kafter The Kakol Be Sar is flying");
//     }

//    fn land(&self) {
//         println!("Kafter The Kakol Be Sar is landing");
//    }

// }

// impl Fly for AirPlane {
//     fn fly(&self) {
//         println!("Airplane is flying.");
//     }

//    fn land(&self) {
//         println!("Airplane is landing.");
//    }
// }

// fn fly_bird(flyable: &Fly) {
//     flyable.fly();
// }

// fn main() {
//     let airplane = AirPlane{};
//     let kakol_be_sar = Kaftar{};
//     fly_bird(&airplane);
//     fly_bird(&kakol_be_sar);
// }

// struct Movie {
//     title: String,
//     director: String,
//     release-year : u32,
//     genre : String 
// }

// trait Details {
//     fn description(&self) -> String;
//     fn years_since_release(&self) -> u32;
// }

// impl Details for Movie {
//     fn description(&self) -> String{
//         return format!("{}, released in {}, is a {} movie directed by {}.", self.title, self.release_year, self.genre, self.director);
//     }

//     fn years_since_release(&self) -> u32 {
//         return 2020 - self.release_year;
//     }
// }

// fn main(){
//     let movie1 = Movie{
//         title: "Titanic".to_string(),
//         director: "James Cameron".to_string(),
//         release_year: 1997,
//         genre: "historical".to_string()
//     };
//     println!("{}", movie1.description());
//     println!("the movie was released {} years ago.", movie1.years_since_release())
// }


use std::clone::Clone;

#[derive(Debug)]
struct YouCanCloneMe {
    name: String,
    age: u8
}


impl Clone for YouCanCloneMe {
    fn clone(&self) -> Self {
        return YouCanCloneMe{name: self.name.clone(), age: self.age};
    }
}

fn main() {
    let yaroo = YouCanCloneMe {
        name: String::from("amir"),
        age: 20
    };
    println!("cloned yaroo: {:#?}", yaroo.clone());
}