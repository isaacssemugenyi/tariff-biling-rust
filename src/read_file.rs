use std::fs::File;
use std::io::stdin;
use std::io::{self, Read};

pub fn prompt_for_receipt_number() {
    let mut receipt_number: String = String::new();

    println!("Enter receipt number: ");

    stdin()
        .read_line(&mut receipt_number)
        .expect("Not a valid receipt number");

    match receipt_number.trim().parse::<u32>() {
        Ok(number) => {
            let result = read_to_file(number);
            match result {
                Ok(_) => println!("Receipt number {} was found and printed", number),
                Err(_) => println!("Receipt number not found"),
            }
        }
        Err(e) => {
            println!("An error occured with provided receipt number, {}", e);
        }
    }
}

pub fn read_to_file(receipt_number: u32) -> io::Result<()> {
    // Example file paths
    let input_file_path = receipt_number.to_string() + ".txt";

    // Read from a file
    let mut input_file = File::open(input_file_path)?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;

    println!("{}", contents);

    Ok(())
}
