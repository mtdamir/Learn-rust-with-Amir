// struct Person { // make a simple struct for a person
//     name : String,
//     real_name: String,
//     height: u8,
//     happiness: bool
// }



// fn main() {
//     let papa_doc = Person { // create variable papa_doc
//         name: "Papa Doc".to_string(),
//         real_name: "Clarence".to_string(),
//         height: 170,
//         happiness: false
//     };

//     let Person { // destructure papa_doc
//         name: a,
//         real_name: b,
//         height: c,
//         happiness: d
//     } = papa_doc;

//     println!("They call him {} but his real name is {}. He is {} cm tall and is he happy? {}", a, b, c, d);

// }

//This prints: They call him Papa Doc but his real name is Clarence. He is 170 cm tall and is he happy? false

//You can see that it's backwards. First we say let papa_doc = Person { fields } to create the struct. 
//Then we say let Person { fields } = papa_doc to destructure it.

//You don't have to write name: a - you can just write name. 
//But here we write name: a because we want to use a variable with the name a.
// --------------------------------------------------------------------------------------------------------------

// Now a bigger example. 
// In this example we have a City struct. 
// We give it a new function to make it. 
// Then we have a process_city_values function to do things with the values. 
// In the function we just create a Vec, but you can imagine that we can do much more after we destructure it.

// struct City {
//     name: String,
//     name_before: String,
//     population: u32,
//     date_founded: u32,
// }

// impl City {
//     fn new(name: String, name_before: String, population: u32, date_founded: u32) -> Self {
//         Self {
//             name,
//             name_before,
//             population,
//             date_founded,
//         }
//     }
// }

// fn process_city_values(city: &City) {
//     let City {
//         name,
//         name_before,
//         population,
//         date_founded,
//     } = city;
//         // now we have the values to use separately
//     let two_names = vec![name, name_before];
//     println!("The city's two names are {:?}", two_names);
// }

// fn main() {
//     let tallinn = City::new("Tallinn".to_string(), "Reval".to_string(), 426_538, 1219);
//     process_city_values(&tallinn);
// }
