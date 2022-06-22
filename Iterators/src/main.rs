//An iterator is a construct that can give you the items in the collection, one at a time. 
//Actually, we have already used iterators a lot: the for loop gives you an iterator. 
//When you want to use an iterator other times, you have to choose what kind:

//.iter() for an iterator of references
//.iter_mut() for an iterator of mutable references
//.into_iter() for an iterator of values (not references)
//A for loop is actually just an iterator that owns its values. 
//That's why it can make it mutable and then you can change the values when you use it

// fn main() {
//     let vector1 = vec![1,2,3]; // we will use .iter() and .into_iter() on this one
//     let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();
//     let vector1_b = vector1.into_iter().map(|x| x * 10).collect::<Vec<i32>>();

//     let mut vector2 = vec![10, 20, 30]; // we will use .iter_mut() on this one
//     vector2.iter_mut().for_each(|x| *x += 100);

//     println!("{:?}", vector1_a);
//     println!("{:?}", vector2);
//     println!("{:?}", vector1_b);
// }

//The first two we used a method called .map(). 
//This method lets you do something to every item, then pass it on. 
//The last one we used is one called .for_each(). This method just lets you do something to every item. 
//.iter_mut() plus for_each() is basically just a for loop. Inside each method we can give a name to every item (we just called it x) and use that to change it. 
//These are called closures and we will learn about them in the next section.

//Let's go over them again, one at a time.

//First we used .iter() on vector1 to get references. 
//We added 1 to each, and made it into a new Vec. vector1 is still alive because we only used references: we didn't take by value. 
//Now we have vector1, and a new Vec called vector1_a. Because .map() just passes it on, we needed to use .collect() to make it into a Vec.

//Then we used into_iter to get an iterator by value from vector1. 
//This destroys vector1, because that's what into_iter() does. 
//So after we make vector1_b we can't use vector1 again.

//Finally we used .iter_mut() for vector2. 
//It is mutable, so we don't need to use .collect() to create a new Vec. 
//Instead, we change the values in the same Vec with mutable references. 
//So vector2 is still there. Because we don't need a new Vec, we use for_each: it's just like a for loop.
// -------------------------------------------------------------------------------------------------------

// Implementing Iterator for your own struct or enum is not too hard. First let's make a book library and think about it.

#[derive(Debug)]
struct Library {
    library_type : LibraryType,
    books: Vec<String>
}

#[derive(Debug)]
enum LibraryType {
    City,
    Country,
}

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.push(book.to_string());
    }

    fn new() -> Self {
        Self {
            library_type:  LibraryType::City,

            books: Vec::new(),
        }
    }
}

fn main() {
    let mut my_library = Library::new(); // make a new library
    my_library.add_book("The Doom of the Darksword"); // add some books
    my_library.add_book("Demian - die Geschichte einer Jugend");
    my_library.add_book("구운몽");
    my_library.add_book("吾輩は猫である");

    println!("{:?}", my_library.books);
}
