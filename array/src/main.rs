// fn main() {
//     let mut my_array : [u8;10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
//     // my_array[5] = 10;
//     println!("my {:?}", my_array);

//     println!("my {:#?}", my_array);

//     println!("my {}", my_array.len());

//     println!("my {}", my_array[5]);
// }

fn main() {
    let my_tuple: (u8, bool, &str) = (10 , true , "salam");
    println!("my tuple {}", my_tuple.1);
}

