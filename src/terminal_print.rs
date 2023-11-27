#[derive(Default)]
pub struct CommercialUnitTuple(pub f32, pub f32, pub f32);

pub struct IndustrialConsumer {
    pub peak: f32,
    pub shoulder: f32,
    pub off_peak: f32,
    pub net_cost: f32,
    pub cost_with_fee: f32,
    pub taxes: f32,
    pub total: f32,
}

impl IndustrialConsumer {
    pub fn new(
        peak: f32,
        shoulder: f32,
        off_peak: f32,
        net_cost: f32,
        cost_with_fee: f32,
        taxes: f32,
        total: f32,
    ) -> Self {
        IndustrialConsumer {
            peak,
            shoulder,
            off_peak,
            net_cost,
            cost_with_fee,
            taxes,
            total,
        }
    }
}

pub struct DomesticConsumer {
    pub net_cost: f32,
    pub cost_with_fee: f32,
    pub taxes: f32,
    pub total: f32,
}

impl DomesticConsumer {
    pub fn new(
        net_cost: f32,
        cost_with_fee: f32,
        taxes: f32,
        total: f32,
    ) -> Self {
        DomesticConsumer {
            net_cost,
            cost_with_fee,
            taxes,
            total,
        }
    }
}

pub fn print_industrial_consumer_to_console(
    results: &IndustrialConsumer,
    consumer_type: &str,
    entered_units: &CommercialUnitTuple,
    costs: (f32, f32, f32),
) {
    println!("|------------------------------------------------------------------------|");
    println!("|Consumer Type: {}\t\t\t\t |", consumer_type);
    println!("|------------------------------------------------------------------------|");
    println!(
        "| {:.2} peak unit @ {:.2} shs costs Ugx {:.2}\t\t\t |",
        entered_units.0, costs.0, results.peak
    );
    println!(
        "| {:.2} shoulder unit @ {:.2} shs costs Ugx {:.2}\t\t\t |",
        entered_units.1, costs.1, results.shoulder
    );
    println!(
        "| {:.2} off peak unit @ {:.2} shs costs Ugx {:.2}\t\t\t |",
        entered_units.2, costs.2, results.off_peak
    );
    println!("|------------------------------------------------------------------------|");
    println!(
        "| The net unit cost is Ugx: \t\t\t\t | {:.2}\t |",
        results.net_cost
    );
    println!(
        "| The net cost with service fee is Ugx:\t\t\t | {:.2}\t |",
        results.cost_with_fee
    );
    println!(
        "| The total taxes is Ugx:\t\t\t\t | {:.2}\t |",
        results.taxes
    );
    println!(
        "| The overall cost is Ugx:\t\t\t\t | {:.2}\t |",
        results.total
    );
    println!("|------------------------------------------------------------------------|");
}

pub fn print_domestic_consumer_to_console(
    domestic_unit_costs: &[f32; 3], 
    domestic_consumer: &DomesticConsumer,
    consumer_type: &str,
    split_units: &CommercialUnitTuple,
    net_costs: &CommercialUnitTuple
){
    println!("|------------------------------------------------------------------------|");
    println!("|Consumer Type: {}\t\t\t\t\t |", consumer_type);
    println!("|------------------------------------------------------------------------|");
    println!(
        "| {:.2} lifeline units @ {:.2} shs costs Ugx {:.2}\t\t\t |",
        split_units.0, domestic_unit_costs[0], net_costs.0
    );
    println!(
        "| {:.2} units btn 16_80 @ {:.2} shs costs Ugx {:.2}\t\t\t |",
        split_units.1, domestic_unit_costs[1], net_costs.1
    );
    println!(
        "| {:.2} units above 81 @ {:.2} shs costs Ugx {:.2}\t\t\t |",
        split_units.2, domestic_unit_costs[2], net_costs.2
    );
    println!("|------------------------------------------------------------------------|");
    println!(
        "| The net unit cost is Ugx: \t\t\t\t | {:.2}\t |",
        domestic_consumer.net_cost
    );
    println!(
        "| The net cost with service fee is Ugx:\t\t\t | {:.2}\t |",
        domestic_consumer.cost_with_fee
    );
    println!(
        "| The total taxes is Ugx:\t\t\t\t | {:.2}\t |",
        domestic_consumer.taxes
    );
    println!(
        "| The overall cost is Ugx:\t\t\t\t | {:.2}\t |",
        domestic_consumer.total
    );
    println!("|------------------------------------------------------------------------|");
}

pub fn print_street_lighting_to_console(
    street_unit_costs: f32, 
    consumed_street_units: f32,
    domestic_consumer: &DomesticConsumer,
    consumer_type: &str
){
    println!("|------------------------------------------------------------------------|");
    println!("|Consumer Type: {}\t\t\t\t\t\t |", consumer_type);
    println!("|------------------------------------------------------------------------|");
    println!(
        "| {:.2} average units @ {:.2} shs costs Ugx {:.2}\t\t\t |",
        consumed_street_units, street_unit_costs, domestic_consumer.net_cost
    );
    println!("|------------------------------------------------------------------------|");
    println!(
        "| The net unit cost is Ugx: \t\t\t\t | {:.2}\t |",
        domestic_consumer.net_cost
    );
    println!(
        "| The net cost with service fee is Ugx:\t\t\t | {:.2}\t |",
        domestic_consumer.cost_with_fee
    );
    println!(
        "| The total taxes is Ugx:\t\t\t\t | {:.2}\t |",
        domestic_consumer.taxes
    );
    println!(
        "| The overall cost is Ugx:\t\t\t\t | {:.2}\t |",
        domestic_consumer.total
    );
    println!("|------------------------------------------------------------------------|");
}