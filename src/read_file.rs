use std::fs::File;
use std::io::{self, Read};

pub fn read_to_file() -> io::Result<()> {
    // Example file paths
    let input_file_path = "input.txt";

    // Read from a file
    let mut input_file = File::open(input_file_path)?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;

    // // Process the contents as needed
    // let processed_contents = contents.to_uppercase();

    println!("{}", contents);

    Ok(())
}
