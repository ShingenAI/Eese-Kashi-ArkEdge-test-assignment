mod tests;
use tests::test_1::get_minutes_until_depletion;
use tests::test_2::{packet_parser};

fn main() {    
    // Problem 1
    println!("\n\nProblem 1\n");
    let result1 = get_minutes_until_depletion("samples/sample1.txt").unwrap();
    println!("Output: (sample1.txt) {}", result1);

    let result2 = get_minutes_until_depletion("samples/sample2.txt").unwrap();
    println!("Output: (sample2.txt): {}", result2);

    // Problem 2
     println!("\n\nProblem 2");
    let _ = packet_parser("samples/sample1.dat");
    let _ = packet_parser("samples/sample2.dat");
}