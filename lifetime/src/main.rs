// Function

// fn print_one<'a>(x: &'a i32) {
//     println!("`print_one`: x is {}", x);
// }

// fn add_one<'a>(x: &'a mut i32) {
//     *x += 1;
// }

// fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
//     println!("`print_multi`: x is {}, y is {}", x, y);
// }


// fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 { x }



// fn main() {
//     let x = 7;
//     let y = 9;
    
//     print_one(&x);
//     print_multi(&x, &y);
    
//     let z = pass_x(&x, &y);
//     print_one(z);

//     let mut t = 3;
//     add_one(&mut t);
//     print_one(&t);
// }
// ======================================================

// Struct
// #[derive(Debug)]
// struct Borrowed<'a>(& 'a i32);

// #[derive(Debug)]
// struct NameBorrowed<'a> {
//     x: &'a i32,
//     y: &'a i32,
// }

// #[derive(Debug)]
// enum Either<'a> {
//     Num(i32),
//     Ref(&'a i32),
// }

// fn main (){
//     let x = 18;
//     let y = 15;

//     let single = Borrowed(&x);
//     let double = NameBorrowed {x: &x, y: &y};
//     let reference = Either::Ref(&x);
//     let number    = Either::Num(y);

//     println!("x is borrowed in {:?}", single);
//     println!("x and y are borrowed in {:?}", double);
//     println!("x is borrowed in {:?}", reference);
//     println!("y is *not* borrowed in {:?}", number);


// }
// ==============================================================

// Static
fn call_me(){
    static mut NUMBER_OF_CALLS: u8 = 0;
    unsafe {
        NUMBER_OF_CALLS += 1;
        println!("number of calls: {}", NUMBER_OF_CALLS);
    }
}

fn main(){
    call_me();
    call_me();
    call_me();
    call_me();
    call_me();
}
