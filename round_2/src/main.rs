mod tests;
use tests::test_1::analyze_battery_logs;
use tests::test_2::schedule_commands;
use tests::test_3::analyze_component_failures;

fn main() {
    // PROBLEM 1
    println!("Problem 1");
    let input = vec![
        "2025-09-17T12:00Z SAT-1 VOLT=3.7",
        "2025-09-17T12:01Z SAT-1 VOLT=3.6",
        "2025-09-17T12:02Z SAT-2 VOLT=3.8",
        "2025-09-17T12:03Z SAT-1 VOLT=3.5",
    ];

    let report =  analyze_battery_logs(&input);
    for line in report {
        println!("{}", line);
    }

    // PROBLEM 2
    let input = vec![(0, 5, 1), (3, 4, 2), (10, 2, 1)];
    let scheduled = schedule_commands(input);

    println!("Problem 2");
    println!("Executed Commands: {:?}", scheduled);

    // PROBLEM 3
    let input = r#"
    [
      {"id":"C-001","type":"OBC","status":"FAILED","failure_code":"E101"},
      {"id":"C-002","type":"OBC","status":"PASSED"},
      {"id":"C-003","type":"Antenna","status":"FAILED","failure_code":"E301"},
      {"id":"C-004","type":"Power","status":"PASSED"}
    ]
    "#;

    println!("Problem 3");
    let report = analyze_component_failures(input);
    for line in report {
        println!("{}", line);
    }
}