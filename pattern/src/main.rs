// fn print_message(user_level: u8){
//     match user_level {
//         0 => println!("Welcome dear admin."),
//         1 => println!("Welcome back our best member."),
//         2 => println!("Hello. Please register first."),
//         _=> println!("Bad Input")
//     }
// }

// fn main() {
//     let user_input: [u8;5] = [0,1,2,3,4];
//     for value in user_input.iter() {
//         print_message(*value);
//     }
// }

// enum Colour {
//     RGB(u16, u16, u16),
//     Hex(String)
// }

// fn print_colour(colour_value: Colour) {
//     match colour_value {
//         Colour::RGB(255, 0, 0) => println!("RED Colour. Here is a completely different code."),
//         Colour::RGB(red, green, blue) => {
//             println!("The colour is in rgb format. red value: {}, green value: {}, blue value: {}", red, green, blue);
//         },
//         Colour::Hex(hex) => println!("The colour is in hex format: #{}", hex)
//     }
// }


// fn main(){
//     let a = Colour::RGB(255, 123, 30);
//     let b = Colour::Hex(String::from("ff7b1e"));
//     let red = Colour::RGB(255, 0, 0);
//     print_colour(a);
//     print_colour(b);
//     print_colour(red);
// }

// fn print_rgb_tuple(rgb: (u16, u16, u16)) {
//     match rgb {
//         (red, 0, 0) => println!("This colour is a gradient of red. red value: {}", red),
//         (red, green, blue) => println!("rgb({}, {}, {})", red, green, blue)
//     }
// }
// fn main() {
//     let r1 = (113, 0, 0);
//     let r2 = (123, 221, 0);
//     print_rgb_tuple(r1);
//     print_rgb_tuple(r2);
// }

// fn main(){
//     let number = 13;

//     println!("Tell me about {}", number);

//     match number {
//         1 => println!("One!"),

//         2 | 3 | 5 | 7 | 11 => println!("This is a prime"),

//         13..=19 => println!("A teen"),

//         _ => println!("Ain't special"),
        
//     }

//     let boolean = true;

//     let binary = match boolean {
//         false => 0,
//         true => 1,
//     };

//     println!("{} -> {}", boolean, binary);

// }
// --------------------------------------------------------------

// enum HttpStatus {
//     Ok = 200,
//     NotFound = 404,
//     InternalServerError = 500
// }

// fn print_status(http_status: HttpStatus){
//     match http_status {
//         HttpStatus::Ok => println!("staus: 200 Ok"),
//         HttpStatus::NotFound => println! ("status: 404"),
//         HttpStatus::InternalServerError => println!("status: 500 Internal Server Error")
//     }
// }

// fn main (){
//     let a = HttpStatus::Ok;
//     let b = HttpStatus::NotFound;
//     let c = HttpStatus::InternalServerError;

//     print_status(a);
//     print_status(b);
//     print_status(c);
// }
// ---------------------------------------------------------------

// struct RgbStruct {
//     red: u16,
//     green: u16,
//     blue: u16
// }

// enum Colour {
//     Transparent,
//     RGB(RgbStruct),
//     Hex(String),
//     CMYK {cyan: f32, magenta: f32, yellow: f32, black: f32}
// }

// fn print_nested_structures(colour: Colour) {
//     match colour {
//         Colour::Transparent => println!("This is Transparent! You can not see anything"),
//         Colour::RGB(RgbStruct{red, green, blue}) => {
//             println!("Colour is in rgb format: ({}, {}, {})", red, green, blue)
//         },
//         Colour::Hex(hex_value) => println!("he colour in the hex format: #{}", hex_value),
//         Colour::CMYK{cyan: c, magenta: m, yellow: y, black: k} => {
//             println!("Colour is in cmyk format: ({}, {}, {}, {})", c, m, y, k)
//         }
//     }
// } 

// fn main(){
//     let t = Colour::Transparent;

//     let rgb = Colour::RGB(RgbStruct{
//         red: 255,
//         green: 255,
//         blue: 255
//     });

//     let hex = Colour::Hex(String::from("ffffff"));

//     let cmyk = Colour::CMYK {
//         cyan: 0.0,
//         magenta: 0.0,
//         yellow: 0.0,
//         black: 0.0
//     };


//     print_nested_structures(t);
//     print_nested_structures(rgb);
//     print_nested_structures(hex);
//     print_nested_structures(cmyk);
// }
// --------------------------------------------------------------

// struct BigStruct {
//     key1: u16,
//     key2: u16,
//     key3: u16,
//     key4: u16,
//     key5: u16,
//     key6: u16,
//     key7: u16,
// }

// fn main () {
//     let a = BigStruct {
//         key1: 0,
//         key2: 1,
//         key3: 2,
//         key4: 3,
//         key5: 4,
//         key6: 5,
//         key7: 6
//     };
//     match a {
//         BigStruct {key1:0 , key7:x, ..} => println!("x = {}",x),
//         BigStruct {key6: y, ..} => println!(" y = {}", y)
//     }
// }
// ------------------------------------------------------------------------------

// struct Foo (u8 , u8);

// fn print_guarded_pattern(input: Foo){
//     match input {
//         Foo(x, _) if x % 2 == 0 => println!("{} is an even number", x),
//         Foo(x , y)  => println!("first value is not even. pair is: ({}, {})", x, y)
//     }
// }

// fn main(){
//     print_guarded_pattern(Foo(2, 10));
//     print_guarded_pattern(Foo(7, 8));
// }
// ---------------------------------------------------------------------------------------

fn get_pair(slice: &[i32]) -> (i32, i32) {
    match slice {
        [e1] => (*e1, *e1),
        [e1, e2, .., e3, e4] => {
            let average1 = (e1 + e2).pow(2);
            let average2 = (e3 + e4).pow(2);
            (average1, average2)
        }
        [e1, .., e2] => (*e1, *e2),
        [] => (0, 0)
    }
}

fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut slice = &a[..];
    let mut pair = get_pair(slice);
    println!("Created pair from {:?} is: {:?}", slice, pair);
    slice = &a[..=1];
    pair = get_pair(slice);
    println!("Created pair from {:?} is: {:?}", slice, pair);
    slice = &a[..=4];
    pair = get_pair(slice);
    println!("Created pair from {:?} is: {:?}", slice, pair);
    slice = &a[0..1];
    pair = get_pair(slice);
    println!("Created pair from {:?} is: {:?}", slice, pair);
    slice = &[];
    pair = get_pair(slice);
    println!("Created pair from {:?} is: {:?}", slice, pair);
    slice = &a[..3];
    pair = get_pair(slice);
    println!("Created pair from {:?} is: {:?}", slice, pair);
}
