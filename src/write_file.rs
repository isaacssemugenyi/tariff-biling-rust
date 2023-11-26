// mod terminal_print;

use crate::terminal_print::*;
use std::fs::File;
use std::io::{self, Read, Write};

// fn write_to_file() -> io::Result<()> 
//     // Example file paths
//     let output_file_path = "output.txt";

//     let mut contents = String::from("New content");

//     // Process the contents as needed
//     let processed_contents = contents.to_uppercase();

//     // Write to a file
//     let mut output_file = File::create(output_file_path)?;
//     output_file.write_all(processed_contents.as_bytes())?;

//     println!("File has been processed and written successfully.");

//     Ok(())
// }

pub fn write_industrial_consumer_to_file(
    results: &IndustrialConsumer,
    consumer_type: &str,
    entered_units: &CommercialUnitTuple,
    costs: (f32, f32, f32),
) -> io::Result<()> {
    let output_file_path = "output.txt";

    let mut output_file = File::create(output_file_path)?;

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
    )?;

    Ok(())
}
