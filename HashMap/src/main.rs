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

// use std::collections::HashMap;

// fn main() {
//     let book_collection = vec!["L'Allemagne Moderne", "Le Petit Prince", "Eye of the World", "Eye of the World"]; // Eye of the World appears twice

//     let mut book_hashmap = HashMap::new();

//     for book in book_collection {
//         book_hashmap.entry(book).or_insert(true);
//     }
//     for (book, true_or_false) in book_hashmap {
//         println!("Do we have {}? {}", book, true_or_false);
//     }
// }
// -------------------------------------------------------------------------------
// Hash Maps and Ownership
// fn main() {
//     use std::collections::HashMap;

//     let field_name = String::from("Favorite color");
//     let field_value = String::from("Blue");

//     let mut map = HashMap::new();
//     map.insert(field_name, field_value);
//     // field_name and field_value are invalid at this point, try using them and
//     // see what compiler error you get!
// }
// -------------------------------------------------------------------------------------------------------------

//You can also do things with .or_insert() like insert a vec and then push into the vec. 
//Let's pretend that we asked men and women on the street what they think of a politician. 
//They give a rating from 0 to 10.

// use std::collections::HashMap;

// fn main() {
//     let data = vec![ // This is the raw data
//         ("male", 9),
//         ("female", 5),
//         ("male", 0),
//         ("female", 6),
//         ("female", 5),
//         ("male", 10)
//     ];

//     let mut survey_hash = HashMap::new();

//     for item in data { // This gives a tuple of (&str, i32)
//         survey_hash.entry(item.0).or_insert(Vec::new()).push(item.1); // This pushes the number into the Vec inside
//     }

//     for (male_or_female, numbers) in survey_hash {
//         println!("{:?}: {:?}", male_or_female, numbers);
//     }
// }

//The important line is: survey_hash.entry(item.0).or_insert(Vec::new()).push(item.1); So if it sees "female" it will check to see if there is "female" already in the HashMap. 
//If not, it will insert a Vec::new(), then push the number in. 
// If it sees "female" already in the HashMap, it will not insert a new Vec, and will just push the number into it
// ---------------------------------------------------------------------------------------------------------------------
// HASHSET

//A HashSet in Rust is a collection that holds a unique value inside it and does not permit the entry of any duplicate values. 
//HashSet is different from HashMap in the sense, that in HashSet, there is no key-value pair and the value or the data inside it, should be directly accessed.

// use std::collections::HashSet;

// fn main() {
//     let many_number = vec![
//         94, 42, 59, 64, 32, 22, 38, 5, 59, 49, 15, 89, 74, 29, 14, 68, 82, 80, 56, 41, 36, 81, 66,
//         51, 58, 34, 59, 44, 19, 93, 28, 33, 18, 46, 61, 76, 14, 87, 84, 73, 71, 29, 94, 10, 35, 20,
//         35, 80, 8, 43, 79, 25, 60, 26, 11, 37, 94, 32, 90, 51, 11, 28, 76, 16, 63, 95, 13, 60, 59,
//         96, 95, 55, 92, 28, 3, 17, 91, 36, 20, 24, 0, 86, 82, 58, 93, 68, 54, 80, 56, 22, 67, 82,
//         58, 64, 80, 16, 61, 57, 14, 11];
    
//         let mut number_hashset = HashSet::new();

//         for number in many_number {
//             number_hashset.insert(number);
//         }

//         let hashset_lenght = number_hashset.len(); // The length tells us how many numbers are in it
//         println!("There are {} unique numbers, so we are missing {}.", hashset_lenght, 100 - hashset_lenght);

//         //let's see what numbers we are missing
//         let mut missing_vec = vec![];
//         for number in 0..100 {
//             if number_hashset.get(&number).is_none() { // If .get() returns None,
//                missing_vec.push(number);
//             }   
//         }

//         print!("It does not contain: ");
//         for number in missing_vec {
//             print!("{} ", number);
//         }
// }

//Here as well, it is easy to change your HashSet to a BTreeSet if you decide you need ordering. 
//In our code, we only need to make two changes to switch from a HashSet to a BTreeSet


// use std::collections::BTreeSet; // Change HashSet to BTreeSet

// fn main() {
//     let many_numbers = vec![
//         94, 42, 59, 64, 32, 22, 38, 5, 59, 49, 15, 89, 74, 29, 14, 68, 82, 80, 56, 41, 36, 81, 66,
//         51, 58, 34, 59, 44, 19, 93, 28, 33, 18, 46, 61, 76, 14, 87, 84, 73, 71, 29, 94, 10, 35, 20,
//         35, 80, 8, 43, 79, 25, 60, 26, 11, 37, 94, 32, 90, 51, 11, 28, 76, 16, 63, 95, 13, 60, 59,
//         96, 95, 55, 92, 28, 3, 17, 91, 36, 20, 24, 0, 86, 82, 58, 93, 68, 54, 80, 56, 22, 67, 82,
//         58, 64, 80, 16, 61, 57, 14, 11];

//     let mut number_btreeset = BTreeSet::new(); // Change HashSet to BTreeSet

//     for number in many_numbers {
//         number_btreeset.insert(number);
//     }
//     for entry in number_btreeset {
//         print!("{} ", entry);
//     }
// }
// --------------------------------------------------------------------------------------------------------
// BinaryHeap
//A BinaryHeap is an interesting collection type, because it is mostly unordered but has a bit of order. 
//It keeps the largest item in the front, but the other items are in any order.

// use std::collections::BinaryHeap;

// fn main() {
//     let mut my_heap = BinaryHeap::new();

//     my_heap.push(500);
//     my_heap.push(125);
//     my_heap.push(-68);
//     my_heap.push(2);
//     my_heap.push(-1);
//     my_heap.push(336);

//     while let  Some(number) = my_heap.pop() {
//         println!("{}", number);
//     }

// }

// Another Example

// use std::collections::BinaryHeap;

// fn show_reminder(input: &BinaryHeap<i32>) -> Vec<i32> {
//     let mut reminder_vec = vec![];
//     for number in input {
//         reminder_vec.push(*number);
//     }
//     reminder_vec
// }

// fn main() {
//     let many_number = vec![0,5,10,15,20,25,30]; //This numbers are in order

//     let mut my_heap = BinaryHeap::new();

//     for number in many_number {
//         my_heap.push(number);
//     }

//     while let Some(number) = my_heap.pop() {
//         println!("Poped off {}, The remaining numbers are: {:?}",
//             number,
//             show_reminder(&my_heap));
//     }

// }
// -------------------------------------------------------------------------------------------
// VecDeque

//A VecDeque is a Vec that is good at popping items both off the front and the back. 
//Rust has VecDeque because a Vec is great for popping off the back (the last item), but not so great off the front. 
//When you use .pop() on a Vec, it just takes off the last item on the right and nothing else is moved. 
//But if you take it off another part, all the items to the right are moved over one position to the left. 
//You can see this in the description for .remove()


