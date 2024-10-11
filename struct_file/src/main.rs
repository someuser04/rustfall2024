use std::fs::File;
use std::io::{Write, BufReader, BufRead};

// Struct to represent a book
struct Book {
    title: String,
    author: String,
    year: u16,
}

// Function to save books to a file
fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
    let mut file = File::create(filename).unwrap(); // Create or overwrite the file
    for book in books {
        writeln!(file, "{},{},{}", book.title, book.author, book.year).unwrap(); // Write each book to the file
    }
}

// Function to load books from a file
fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
    let file = File::open(filename).unwrap(); // Open the file for reading
    let reader = BufReader::new(file);

    let mut books = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap(); // Read each line
        let parts: Vec<&str> = line.split(',').collect(); // Split the line into parts
        if parts.len() == 3 {
            let title = parts[0].to_string();
            let author = parts[1].to_string();
            let year = parts[2].parse::<u16>().unwrap(); // Parse the year as u16
            books.push(Book { title, author, year }); // Add the book to the list
        }
    }

    books
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    // Save the books to a file
    save_books(&books, "books.txt");
    println!("Books saved to file.");

    // Load the books from the file
    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}
