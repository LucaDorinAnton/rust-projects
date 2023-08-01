use std::fs;
use std::io;
use std::error::Error;


fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}


fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() -> Result<(), Box<dyn Error>>{
    let greeting_file = fs::File::open("hello.txt")?;

    let text = "";

    println!("The last char of the text is: {}", last_char_of_first_line(text).unwrap());
    
    Ok(())
}
