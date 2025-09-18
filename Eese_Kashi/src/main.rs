mod tests;
use tests::test_1::get_minutes_until_depletion;
use tests::test_2::{packet_parser};

fn ceil_div_nonneg(x: i64, d: i64) -> i64 {
    if x <= 0 { 0 } else { (x + d - 1) / d }
}

fn main() {
    let n = ceil_div_nonneg(180, 20);
    println!("n = {}", n); // should be 9
    
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