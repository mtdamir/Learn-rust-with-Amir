// #[derive(Debug)]
// struct Student {
//     name: String,
//     id: u32,
//     graudate: bool
// }


// fn main() {
//     let mut amir = Student {
//         id: 21467994,
//         name: String::from("Amir Abbas Motavas"),
//         graudate: true
//     };
//     // println!("{:?}", amir );
//     // println!("{:?}", amir.name );
//     amir.name = String::from("amir motavas");
//     println!("Student name: {}", amir.name);
// }

// Factory_Function

#[derive(Debug)]
struct Student {
    name: String,
    id: u32,
    graudate: bool
}

fn create_Student(name: String, id:u32, graudate:
    bool) -> Student{
        Student{
            name,
            id,
            graudate,
        }
    }

    fn main(){
        let amir = create_Student (String::from("AMir motavas"),353698559, true);
        println!("Student name: {:?}", amir)
    }
