mod tests;
use tests::test_1::minutes_until_depletion;
use tests::test_2::packet_parser;

fn main() {
    // Problem 1
    let (a, b, c) = (40_i64, 30_i64, 20_i64);  //  sample input from Problem 1
    let result = minutes_until_depletion(a, b, c);
    println!("{}", result);  // Result
    
    // Problem 2
    let _ = packet_parser("samples/sample1.dat");
    let _ = packet_parser("samples/sample2.dat");
}