use std::env;
use std::process;

use dobble_builder::create;

fn main() {
    let args: Vec<String> = env::args().collect();
    let symbols_per_card: u8 = parse_config(&args);

    if let Err(e) = create(symbols_per_card) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn parse_config(args: &[String]) -> u8 {
    let symbols_per_card: u8 = args[1].trim().parse().expect("Please provide a valid number");
    symbols_per_card
}

