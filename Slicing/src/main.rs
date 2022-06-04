fn main() {
    let mut my_array  = [-10, -3, -2, 0, 9, 3, 9];

    let not_negative_element = first_not_negative_element(my_array);
    if not_negative_element > -1 {
        println!("first_not_negative_element: {}", not_negative_element)
    }else{
        println!{"all of my arry is negatove"}
    }

    my_array = [10;7];
    println!("incorect element {}",not_negative_element)
    
}


fn first_not_negative_element(arry: [i32;7]) -> i32 {
    for(_index , &item) in arry.iter().enumerate(){
        if item > -1 {
            return item;
        }
    }
    return -1;
}
