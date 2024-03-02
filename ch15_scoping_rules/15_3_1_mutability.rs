// Mutable data can be borrowed using &mut T
// Called mutable reference, gives read/write access to borrower
// &T borrows data via immutable reference -> borrower reads data but does not borrow it


#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` is ref to string allocated in read only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

// Function takes reference to book
fn borrow_book(book: &Book) {
    println!("Immutably borrowed {} - {} edition", book.title, book.year);
}

// Function takes reference to mutable book and changes `year` to 2014
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    // Create immutable Book
    let immutabook = Book {
        // string literals have type `&'static str`
        author: "Douglas Hofstadter",
        title: "Godel, Escher, Bach",
        year: 1979,
    };

    // Create mutable copy of `immutabook`
    let mut mutabook = immutabook;

    // Immutably borrow an immutable object
    borrow_book(&immutabook);

    // Immutably borrow mutable object
    borrow_book(&mutabook);

    // Borrow mutable object as mutable
    new_edition(&mut mutabook);

    // Error! Cannot borrow immutable object as mutable
    // new_edition(&mut immutabook);
}