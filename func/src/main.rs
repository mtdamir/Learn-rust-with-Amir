fn main(){
    example(100);
    println!("2 * 2 = {}", muliple(2));
    println!("10! = {}", factorial(10));
}

fn example(number: i16){
    println!("number: {}", number);
}

fn muliple(number: u8) -> u8{
    return number * 2;
}

fn factorial(n: u64) -> u64 {
    if n <= 1 {
       return 1
    }
    return n * factorial(n - 1);
}
