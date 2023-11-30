use crate::{terminal_print::*, get_print_main_menu};

use std::fs::*;
use std::io::{self, stdin, Write};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn check_user_meter_number() {
    let mut customer_option: String = String::new();

    println!("Are you a new customer or an existing customer: ");
    println!("1. Yes");
    println!("2. No");

    stdin()
        .read_line(&mut customer_option)
        .expect("Not a valid selection");

    match customer_option.trim().parse::<u8>() {
        Ok(option) => {
            match option {
                1 => {
                    // prompt user to input meter number
                    let mut folder_name: String = String::new();
                    println!("Great! Please enter your meter number: ");

                    stdin()
                        .read_line(&mut folder_name)
                        .expect("Not a valid selection");

                    let result = check_directory_exists(folder_name.trim());

                    match result {
                        Ok(_) => {
                            match result {
                                Ok(_) => {
                                     get_print_main_menu(folder_name.trim());

                                    // concatenate the meter number with the generated receipt number
                                    // save
                                }
                                Err(e) => println!("{}", e),
                            }
                        },
                        Err(_) => print!("Meter number {} not found", folder_name),
                    }
                }
                2 => {
                    let mut choosen_option: String = String::new();

                    println!("Do you want to become a customer: ");
                    println!("1. Yes");
                    println!("2. No");

                    stdin()
                        .read_line(&mut choosen_option)
                        .expect("Not a valid selection");

                    match choosen_option.trim().parse::<u8>() {
                        Ok(current_option) => match current_option {
                            1 => {
                                let (_, generated_meter_number) = generate_random_receipt_numbers();
                                create_user_meter_number(generated_meter_number.to_string().trim());
                            }
                            2 => println!("Thank you for using visiting us and enjoy your day"),
                            _ => println!("The option select is not a valid selection")
                        },
                        Err(e) => println!("Invalid input {}", e),
                    }
                }
                _ => println!("Unknown option selected"),
            }
        }
        Err(e) => {
            println!("An error occured with provided receipt number, {}", e);
        }
    }
}

fn create_user_meter_number(folder_name: &str) {
    match create_dir(folder_name) {
        Ok(_) => {
            println!("Account/ Meter number '{}' created successfully", folder_name);
        }
        Err(e) => {
            eprintln!("Error generating meter number {}", e);
        }
    }
}

fn check_directory_exists(meter_number: &str) -> io::Result<()> {
    let exists_result = metadata(meter_number)?;

    match exists_result.is_dir() {
        true => println!("Meter number {} exists", meter_number),
        false => println!("Meter number {} is not registered.", meter_number),
    }
    Ok(())
}

// https://users.rust-lang.org/t/random-number-without-using-the-external-crate/17260/8
// Solution got from above link
fn generate_random_receipt_numbers() -> (String, u32) {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    (nanos.to_string() + ".txt", nanos)
}

pub fn write_industrial_consumer_to_file(
    meter_number: &str,
    results: &IndustrialConsumer,
    consumer_type: &str,
    entered_units: &CommercialUnitTuple,
    costs: (f32, f32, f32),
) -> io::Result<u32> {
    let (file_name, receipt_number) = generate_random_receipt_numbers();

    let file_path = meter_number.to_owned()+"/"+&file_name;

    let mut output_file = File::create(file_path)?;

    write!(
        &mut output_file,
        "
    |----------------------------------------------------------------|
    |Consumer Type: {}\t\t\t\t\t\t |
    |----------------------------------------------------------------|
    | {:.2} peak unit @ {:.2} shs costs Ugx {:.2}\t\t\t\t |
    | {:.2} shoulder unit @ {:.2} shs costs Ugx {:.2}\t\t\t |
    | {:.2} off peak unit @ {:.2} shs costs Ugx {:.2}\t\t\t |
    |----------------------------------------------------------------|
    | The net unit cost is Ugx: \t\t\t\t\t\t | {:.2}\t |
    | The net cost with service fee is Ugx:\t\t\t\t | {:.2}\t |
    | The total taxes is Ugx:\t\t\t\t\t\t\t | {:.2}\t |
    | The overall cost is Ugx:\t\t\t\t\t\t\t | {:.2}\t |
    |----------------------------------------------------------------|
    ",
        consumer_type,
        entered_units.0,
        costs.0,
        results.peak,
        entered_units.1,
        costs.1,
        results.shoulder,
        entered_units.2,
        costs.2,
        results.off_peak,
        results.net_cost,
        results.cost_with_fee,
        results.taxes,
        results.total,
    )
    .expect("Failed to write to file");

    Ok(receipt_number)
}

pub fn write_domestic_consumer_to_file(
    meter_number: &str,
    domestic_unit_costs: &[f32; 3],
    domestic_consumer: &DomesticConsumer,
    consumer_type: &str,
    split_units: &CommercialUnitTuple,
    net_costs: &CommercialUnitTuple,
) -> io::Result<u32> {
    let (file_name, receipt_number) = generate_random_receipt_numbers();

    let file_path = meter_number.to_owned()+"/"+&file_name;

    let mut output_file = File::create(file_path)?;

    write!(
        &mut output_file,
        "
    |----------------------------------------------------------------|
    |Consumer Type: {}\t\t\t\t\t\t\t\t |
    |----------------------------------------------------------------|
    | {:.2} lifeline units @ {:.2} shs costs Ugx {:.2}\t\t\t |
    | {:.2} units btn 16_80 @ {:.2} shs costs Ugx {:.2}\t\t\t |
    | {:.2} units above 81 @ {:.2} shs costs Ugx {:.2}\t\t\t |
    |----------------------------------------------------------------|
    | The net unit cost is Ugx: \t\t\t\t\t\t | {:.2}\t |
    | The net cost with service fee is Ugx:\t\t\t\t | {:.2}\t |
    | The total taxes is Ugx:\t\t\t\t\t\t\t | {:.2}\t |
    | The overall cost is Ugx:\t\t\t\t\t\t\t | {:.2}\t |
    |----------------------------------------------------------------|
    ",
        consumer_type,
        split_units.0,
        domestic_unit_costs[0],
        net_costs.0,
        split_units.1,
        domestic_unit_costs[1],
        net_costs.1,
        split_units.2,
        domestic_unit_costs[2],
        net_costs.2,
        domestic_consumer.net_cost,
        domestic_consumer.cost_with_fee,
        domestic_consumer.taxes,
        domestic_consumer.total,
    )?;

    Ok(receipt_number)
}

pub fn write_street_lighting_to_file(
    meter_number: &str,
    street_unit_costs: f32,
    consumed_street_units: f32,
    domestic_consumer: &DomesticConsumer,
    consumer_type: &str,
) -> io::Result<u32> {
    let (file_name, receipt_number) = generate_random_receipt_numbers();

    let file_path = meter_number.to_owned()+"/"+&file_name;

    let mut output_file = File::create(file_path)?;

    write!(
        &mut output_file,
        "
    |----------------------------------------------------------------|
    |Consumer Type: {}\t\t\t\t\t\t\t\t |
    |----------------------------------------------------------------|
    | {:.2} average units @ {:.2} shs costs Ugx {:.2}\t\t\t |
    |----------------------------------------------------------------|
    | The net unit cost is Ugx: \t\t\t\t\t\t | {:.2}\t |
    | The net cost with service fee is Ugx:\t\t\t\t | {:.2}\t |
    | The total taxes is Ugx:\t\t\t\t\t\t\t | {:.2}\t |
    | The overall cost is Ugx:\t\t\t\t\t\t\t | {:.2}\t |
    |----------------------------------------------------------------|
    ",
        consumer_type,
        consumed_street_units,
        street_unit_costs,
        domestic_consumer.net_cost,
        domestic_consumer.net_cost,
        domestic_consumer.cost_with_fee,
        domestic_consumer.taxes,
        domestic_consumer.total,
    )?;

    Ok(receipt_number)
}
