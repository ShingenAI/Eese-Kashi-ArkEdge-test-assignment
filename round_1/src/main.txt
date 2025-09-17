mod tests;
use tests::{test_1, test_2, test_3};


use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("1") => test_1::telemetry_parser::run(),
        Some("2") => test_2::scheduler::run(),
        Some("3") => test_3::traceability::run(),
        _ => {
            println!("Usage: cargo run [1|2|3]");
        }
    }
}