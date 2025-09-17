# Rust Eese Kashi | test-1

These solutions are based on the ArkeEdge Space Project Manager coding test, focusing on correctness, clarity, and modular design.

# Test Recursion Summary

## Test 1: Telemetry Parser
- Parses live satellite telemetry.
- Tracks latest parameter and TEMP averages.
...

## Test 2: Command Scheduler
- Reads command windows.
- Ensures no overlapping runs.
...

## Test 3: Traceability
- Aggregates pass/fail counts by component type.
...

Each test is:
- Fully modular
- Executable via `main.rs`
- Teachable for future engineers

## How to Run

```bash
cargo run