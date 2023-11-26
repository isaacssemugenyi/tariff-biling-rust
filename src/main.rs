mod terminal_print;

use std::io::stdin;
use terminal_print::*;

fn main() {
    get_print_main_menu();
}

fn get_print_main_menu() {
    let mut selected_option: String = String::new();
    let menu_options: [&str; 8] = [
        "Domestic Consumer",
        "Commercial Consumer",
        "Medium-industrial Consumer",
        "Large-industrial Consumer",
        "Street Lighting",
        "Print recent receipt",
        "Save a receipt",
        "Print personal receipts",
    ];

    println!("Select an option");
    for (i, option) in menu_options.iter().enumerate() {
        println!("{}. {}", i + 1, option);
    }

    stdin()
        .read_line(&mut selected_option)
        .expect("Not a valid string");

    match selected_option.trim().parse::<u8>() {
        Ok(i) => choose_selected_option(i, &menu_options[(i -1) as usize]),
        Err(_) => {
            println!("Invalid option. Try again");
            get_print_main_menu()
        }
    }
}

fn choose_selected_option(selected_option: u8, menu_options: &str) {
    match selected_option {
        1 => input_domestic_consumer_units(menu_options),
        2..=4 => calc_consumption_for_industrial_consumer(selected_option, menu_options),
        5 => input_street_light_units(menu_options),
        6 => println!("You selected 6"),
        7 => println!("You selected 7"),
        8 => println!("You selected 8"),
        _ => {
            println!("Invalid option. Try again");
            get_print_main_menu()
        }
    }
}

fn calc_consumption_for_industrial_consumer(option: u8, consumer_type: &str) {
    /*
      Domestic - {lifeline: 250.00, 16-80: 805.00, >81: 412.00}
      street-lighting - {average: 370.00}
    */

    match option {
        2 | 3 | 4 => {
            let costs: [(f32, f32, f32); 3] = [
                (807.50, 615.50, 367.40), // commercial - {peak: 807.50, shoulder: 615.50, off_peak: 367.40}
                (519.40, 385.30, 243.60), // medium - {peak: 519.40, shoulder: 385.30, off_peak: 243.60}
                (497.00, 368.60, 233.10), // large - {peak: 497.00, shoulder: 368.60, off_peak: 233.10}
            ];

            let result: CommercialUnitTuple = get_industrial_consumer_consumption();

            let output: IndustrialConsumer =
                compute_industrial_cost(&result, costs[(option - 2) as usize]);

            print_industrial_consumer_to_console(
                output,
                consumer_type,
                &result,
                costs[(option - 2) as usize],
            );
        }
        _ => println!("Unsupported selection."),
    }
}

fn get_industrial_consumer_consumption() -> CommercialUnitTuple {
    let mut peak_units: String = String::new();
    let mut shoulder_units: String = String::new();
    let mut off_peak_units: String = String::new();

    let mut used_units: CommercialUnitTuple = Default::default();

    println!("Enter peak units: ");

    stdin()
        .read_line(&mut peak_units)
        .expect("Not a valid unit");

    match peak_units.trim().parse::<f32>() {
        Ok(peak) => {
            if peak >= 0.0 {
                used_units.0 = peak
            } else {
                println!("Not a valid unit");
                get_print_main_menu()
            }
        }
        Err(_) => println!("Not a valid unit"),
    }

    println!("Enter shoulder units: ");
    stdin()
        .read_line(&mut shoulder_units)
        .expect("Not a valid unit");

    match shoulder_units.trim().parse::<f32>() {
        Ok(shoulder) => {
            if shoulder >= 0.0 {
                used_units.1 = shoulder
            } else {
                println!("Not a valid unit");
                get_print_main_menu()
            }
        }
        Err(_) => println!("Not a valid unit"),
    }

    println!("Enter off peak units: ");
    stdin()
        .read_line(&mut off_peak_units)
        .expect("Not a valid unit");

    match off_peak_units.trim().parse::<f32>() {
        Ok(off_peak) => {
            if off_peak >= 0.0 {
                used_units.2 = off_peak
            } else {
                println!("Not a valid unit");
                get_print_main_menu()
            }
        }
        Err(_) => println!("Not a valid unit"),
    }

    used_units
}

fn compute_industrial_cost(
    units: &CommercialUnitTuple,
    unit_cost: (f32, f32, f32),
) -> IndustrialConsumer {
    let peak_unit_cost = units.0 * unit_cost.0;
    let shoulder_unit_cost = units.1 * unit_cost.1;
    let off_peak_unit_cost = units.2 * unit_cost.2;
    let net_cost = peak_unit_cost + shoulder_unit_cost + off_peak_unit_cost;
    let cost_service_fee = net_cost + 3360.0;
    let taxes = cost_service_fee * 18.0 / 100.0;
    let total_cost_with_taxes = cost_service_fee + taxes;

    IndustrialConsumer::new(
        peak_unit_cost,
        shoulder_unit_cost,
        off_peak_unit_cost,
        net_cost,
        cost_service_fee,
        taxes,
        total_cost_with_taxes,
    )
}

fn input_domestic_consumer_units(consumer_type: &str) {
    let mut consumed_units: String = String::new();

    println!("Enter consumed units: ");

    stdin()
        .read_line(&mut consumed_units)
        .expect("Not a valid unit");

    match consumed_units.trim().parse::<f32>() {
        Ok(unit) => {
            if unit >= 0.0 {
                calculate_domestic_consumer_costs(unit, consumer_type)
            } else {
                println!("Invalid option. Try again");
                get_print_main_menu()
            }
        }
        Err(_) => {
            println!("Invalid option. Try again");
            get_print_main_menu()
        }
    }
}

fn calculate_domestic_consumer_costs(units: f32, consumer_type: &str) {
    let domestic_unit_costs: [f32; 3] = [250.00, 805.00, 412.00];
    let mut split_units: CommercialUnitTuple = Default::default(); // tuple to keep units for below 15, between 16 to 80 and above 80
    let mut net_costs: CommercialUnitTuple = Default::default(); // tuple to store the unit cost for the first 15 units, between 16 to 80 and aboe 80

    let mut cost_btn_16_80: f32 = 0.0;
    let mut lifeline: f32 = 0.0;
    let mut above_81: f32 = 0.0;

    if units <= 100.0 {
        if units <= 15.0 {
            lifeline = units * domestic_unit_costs[0];
            split_units.0 = units;
            net_costs.0 = lifeline;
        } else if units <= 80.0 {
            lifeline = 15.0 * domestic_unit_costs[0];
            cost_btn_16_80 = (units - 15.0) * domestic_unit_costs[1];

            split_units.0 = 15.0;
            split_units.1 = units - 15.0;

            net_costs.0 = lifeline;
            net_costs.1 = cost_btn_16_80;
        } else {
            let remainder_above_81: f32 = units - 15.0 - 65.0; // (85 - 15) = 70, 70 - 65 = 5;
            lifeline = 15.0 * domestic_unit_costs[0];
            cost_btn_16_80 = 65.0 * domestic_unit_costs[1];
            above_81 = remainder_above_81 * domestic_unit_costs[2];

            split_units.0 = 15.0;
            split_units.1 = 65.0;
            split_units.2 = remainder_above_81;

            net_costs.0 = lifeline;
            net_costs.1 = cost_btn_16_80;
            net_costs.2 = above_81;
        }
    } else {
        cost_btn_16_80 = 65.0 * domestic_unit_costs[1];
        above_81 = (units - 65.0) * domestic_unit_costs[2];

        split_units.0 = 0.0;
        split_units.1 = 65.0;
        split_units.2 = units - 65.0;

        net_costs.0 = 0.0;
        net_costs.1 = cost_btn_16_80;
        net_costs.2 = above_81;
    }

    let net_total_without_service_fee: f32 = cost_btn_16_80 + lifeline + above_81;
    let sub_total_with_service_fee: f32 = net_total_without_service_fee + 3360.0;
    let vat_amount: f32 = (18.0 / 100.0) * sub_total_with_service_fee;
    let total: f32 = vat_amount + sub_total_with_service_fee;

    let domestic_consumer = DomesticConsumer::new(
        net_total_without_service_fee,
        sub_total_with_service_fee,
        vat_amount,
        total,
    );

    print_domestic_consumer_to_console(
        &domestic_unit_costs,
        &domestic_consumer,
        consumer_type,
        &split_units,
        &net_costs,
    );
}

fn input_street_light_units(consumer_type: &str) {
    let mut average_units: String = String::new();

    println!("Enter the Average Units consumed: ");

    stdin()
        .read_line(&mut average_units)
        .expect("Not a valid unit");

    match average_units.trim().parse::<f32>() {
        Ok(unit) => {
            if unit >= 0.0 {
                calculate_street_light_consumption(unit, consumer_type)
            } else {
                println!("Invalid option. Try again");
                get_print_main_menu()
            }
        }
        Err(_) => {
            println!("Invalid option. Try again");
            get_print_main_menu()
        }
    }
}

fn calculate_street_light_consumption(unit: f32, consumer_type: &str) {
    let average_unit_cost: f32 = 370.00;
    let net_unit_cost: f32 = unit * average_unit_cost; // multiply the average_units by single unit cost
    let total_cost_with_service_fee: f32 = net_unit_cost + 3360.0; // add total unit cost + service fee
    let vat_amount: f32 = total_cost_with_service_fee * (18.0 / 100.0); // calculate the vat cost
    let total_cost_with_vat_and_service_fee: f32 = total_cost_with_service_fee + vat_amount; // total cost with vat

    let domestic_consumer = DomesticConsumer::new(
        net_unit_cost,
        total_cost_with_service_fee,
        vat_amount,
        total_cost_with_vat_and_service_fee,
    );

    print_street_lighting_to_console(
        average_unit_cost,
        unit,
        &domestic_consumer,
        consumer_type,
    )
}
