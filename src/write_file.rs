use crate::terminal_print::*;

use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::io::{self, Write};


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
    results: &IndustrialConsumer,
    consumer_type: &str,
    entered_units: &CommercialUnitTuple,
    costs: (f32, f32, f32),
) -> io::Result<u32> {
    let ( file_name, receipt_number ) = generate_random_receipt_numbers();

    let mut output_file = File::create(file_name)?;

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
    entered_units.0, costs.0, results.peak,
    entered_units.1, costs.1, results.shoulder,
    entered_units.2, costs.2, results.off_peak,
    results.net_cost,
    results.cost_with_fee,
    results.taxes,
    results.total,
    ).expect("Failed to write to file");

    Ok(receipt_number)
}


pub fn write_domestic_consumer_to_file(
    domestic_unit_costs: &[f32; 3], 
    domestic_consumer: &DomesticConsumer,
    consumer_type: &str,
    split_units: &CommercialUnitTuple,
    net_costs: &CommercialUnitTuple
) -> io::Result<u32> {
    let ( file_name, receipt_number ) = generate_random_receipt_numbers();

    // let output_file_path = generate_random_receipt_numbers().0;
    // let output_file_path = "output.txt";

    let mut output_file = File::create(file_name)?;

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
    split_units.0, domestic_unit_costs[0], net_costs.0,
    split_units.1, domestic_unit_costs[1], net_costs.1,
    split_units.2, domestic_unit_costs[2], net_costs.2,
    domestic_consumer.net_cost,
    domestic_consumer.cost_with_fee,
    domestic_consumer.taxes,
    domestic_consumer.total,
    )?;

    Ok(receipt_number)
}


pub fn write_street_lighting_to_file(
    street_unit_costs: f32, 
    consumed_street_units: f32,
    domestic_consumer: &DomesticConsumer,
    consumer_type: &str
) -> io::Result<u32> {
    let ( file_name, receipt_number ) = generate_random_receipt_numbers();

    let mut output_file = File::create(file_name)?;

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
    consumed_street_units, street_unit_costs, domestic_consumer.net_cost,
    domestic_consumer.net_cost,
    domestic_consumer.cost_with_fee,
    domestic_consumer.taxes,
    domestic_consumer.total,
    )?;

    Ok(receipt_number)
}