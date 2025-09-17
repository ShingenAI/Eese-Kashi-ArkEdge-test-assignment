mod tests;


fn main() {
    println!("Test 1 – Telemetry Parser:");
    tests::test_1::telemetry_parser::run_telemetry_parser();
    
    println!("Test 2 – Command Scheduler:");
    tests::test_2::scheduler::run();

    println!("Test 3 -  Manufacturing Traceability:");
    tests::test_3::traceability::run();
}