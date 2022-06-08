//A HashMap is a collection made out of keys and values. 
//You use the key to look up the value that matches the key. 
//You can create a new HashMap with just HashMap::new() and use .insert(key, value) to insert items.
// use std::collections::HashMap; // This is so we can just write HashMap instead of std::collections::HashMap every time

// struct City {
//     name: String,
//     population: HashMap<u32, u32>, // This will have the year and the population for the year
// }

// fn main() {

//     let mut tallinn = City {
//         name: "Tallinn".to_string(),
//         population: HashMap::new(), // So far the HashMap is empty
//     };

//     tallinn.population.insert(1372, 3_250); // insert three dates
//     tallinn.population.insert(1851, 24_000);
//     tallinn.population.insert(2020, 437_619);


//     for (year, population) in tallinn.population { // The HashMap is HashMap<u32, u32> so it returns a two items each time
//         println!("In the year {} the city of {} had a population of {}.", year, tallinn.name, population);
//     }
// }
// ------------------------------------------------------------------------------------------------------------

// BTreeMap
// use std::collections::BTreeMap;

// struct City{   // This is a struct
//     name: String,
//     population: BTreeMap<u32, u32>, // This is a BTreeMap
// }

// fn main() {
//     let mut tallinn = City {
//         name: "Tallinn".to_string(),
//         population: BTreeMap::new(),
//     };

//     tallinn.population.insert(1372, 3_250);
//     tallinn.population.insert(1851, 24_000);
//     tallinn.population.insert(2020, 437_619);

//     for (year, population) in tallinn.population {
//         println!("In the year {} the city of {} had a population of {}.", year, tallinn.name, population);
//     }
    
// }
// ---------------------------------------------------------------------------------------------------------------------

// use std::collections::HashMap;

// fn main() {
//     let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
//     let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];

//     let mut city_hashmap = HashMap::new();

//     for city in canadian_cities {
//         city_hashmap.insert(city, "Canada");
//     }
//     for city in german_cities {
//         city_hashmap.insert(city, "Germany");
//     }

//     println!("{:?}", city_hashmap["Bielefeld"]);
//     println!("{:?}", city_hashmap.get("Bielefeld"));
//     println!("{:?}", city_hashmap.get("Bielefeldd"));
// }
// ---------------------------------------------------------------------------------------------------------------------

//HashMap has a very interesting method called .
//entry() that you definitely want to try out. 
//With it you can try to make an entry and use another method like .
//or_insert() to insert the value if there is no key. 
//The interesting part is that it also gives a mutable reference so you can change it if you want. 
//First is an example where we just insert true every time we insert a book title into the HashMa

use std::collections::HashMap;

fn main() {
    let book_collection = vec!["L'Allemagne Moderne", "Le Petit Prince", "Eye of the World", "Eye of the World"]; // Eye of the World appears twice

    let mut book_hashmap = HashMap::new();

    for book in book_collection {
        book_hashmap.entry(book).or_insert(true);
    }
    for (book, true_or_false) in book_hashmap {
        println!("Do we have {}? {}", book, true_or_false);
    }
}

