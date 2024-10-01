use std::io::{self, Write, Read};
use std::fs::File;
struct Car{
    model: String,
    year: u32,
}
fn creating_file() -> File
{
    File::create("user_info.txt").unwrap()
}
fn reading_from_console(file: &mut File)
{
    let mut buffer = String::new();

    print!("What model is your car? \n");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let model = buffer.trim().to_string();
    buffer.clear();

    print!("What year is your car? \n");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year = buffer.trim().parse().unwrap();

    let car = Car {model, year};

    writeln!(file, "{}", car.model).unwrap();
    writeln!(file, "{}", car.year).unwrap();
}
fn reading_file() 
{
    let mut f = File::open("user_info.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    println!("Contents of the file \n{}", contents);
}
fn main()
{
    let mut file = creating_file();
    reading_from_console(&mut file);
    reading_file();
}