use std::fs::File;
use std::io::stdin;
use std::io::{self, Read};

pub fn prompt_for_receipt_number(meter_number: &str) {
    let mut receipt_number: String = String::new();

    println!("Enter receipt number: ");

    stdin()
        .read_line(&mut receipt_number)
        .expect("Not a valid receipt number");

    match receipt_number.trim().parse::<u32>() {
        Ok(number) => {
            let result = read_to_file(meter_number, number);
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

pub fn read_to_file(meter_number: &str, receipt_number: u32) -> io::Result<()> {
    // Example file paths
    let receipt_number_string = receipt_number.to_string() + ".txt";
    let file_path = format!("{}/{}", meter_number, receipt_number_string);

    // Read from a file
    let mut input_file = File::open(file_path)?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;

    println!("{}", contents);

    Ok(())
}
