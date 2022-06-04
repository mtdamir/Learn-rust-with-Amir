// fn main() {
//     let empty_String = String::new();
//     println!("Lenght is {}", empty_String.len());

//     let content_string = String::from("TutorialsPoint");
//     println!("Lenght is {}", content_string.len())
// }
// -----------------------------------------------------------------
//push_str

// fn main() {
//     let mut z = String::new();
//     z.push_str("salam");
//     println!("{}", z)
// }
// -----------------------------------------------------------------

// to_stirng
// fn main(){
//     let name1 = "Hello TutorialsPoint , 
//     Hello!".to_string();
//     println!("{}",name1);
//  }
// -----------------------------------------------------------------

// as_str
// fn main(){
//     let example_string = String::from("example_string");
//     print_literal(example_string.as_str())
// }

// fn print_literal(data:&str ){
//     println!("displaying string literal {}",data);
// } 


fn main(){
    for x in 1..11{ // 11 is not inclusive
       if x==5 {
          continue;
       }
       println!("x is {}",x);
    }
 }
