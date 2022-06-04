//We learned that when you have a reference, you need to use * to get to the value. 
//A reference is a different type, so this won't work:

//So we change line 5 to println!("{}", my_number == *reference); 
//and now it prints true because it's now i32 == i32, not i32 == &i32. 
//This is called dereferencing.

// fn main() {
//     let my_number = 9;
//     let reference = &my_number;

//     // println!("{}", my_number == reference);
//     println!("{}", my_number == *reference); 
 
// }
// ---------------------------------------------------------------------------

// But when you use a method, Rust will dereference for you. 
// The ( . ) in a method is called the dot operator, and it does dereferencing for free.

// First, let's make a struct with one u8 field. 
// Then we will make a reference to it and try to compare. It will not work

// To make it work, we need to dereference: println!("{}", *reference_number == 8);.

// But with the dot operator, we don't need *

// struct Item {
//     number: u8,
// }

// fn main() {
//     let item = Item {
//         number: 8,
//     };

//     // let reference_number = &item.number; // reference number type is &u8

//     // println!("{}", reference_number == 8);

//     let reference_item = &item;

//     println!("{}", reference_item.number == 8); // we don't need to write *reference_item.number
// }
// ----------------------------------------------------------------------------------------------------------

// Now let's create a method for Item that compares number to another number. 
// We don't need to use * anywhere

struct Item {
    number: u8,
}

impl Item {
    fn compare_number(&self, other_number: u8) { // takes a reference to self
        println!("Are {} and {} equal? {}", self.number, other_number, self.number == other_number);
            // We don't need to write *self.number
    }
}

fn main() {
    let item = Item {
        number: 8,
    };

    let reference_item = &item; // This is type &Item
    let reference_item_two = &reference_item; // This is type &&Item

    item.compare_number(8); // the method works
    reference_item.compare_number(8); // it works here too
    reference_item_two.compare_number(8); // and here

}
