use std::collections::BTreeMap;

// tests / test_1 / mod.rs
/// Analyzes satellite battery logs and returns formatted statistics per satellite.
pub fn analyze_battery_logs(logs: &[&str]) -> Vec<String> {
    let mut data: BTreeMap<String, Vec<f32>> = BTreeMap::new();

    for line in logs {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 3 {
            continue; // Skip malformed lines
        }
        let satellite_id = parts[1];
        if let Some(voltage_str) = parts[2].strip_prefix("VOLT=") {
            if let Ok(voltage) = voltage_str.parse::<f32>() {
                data.entry(satellite_id.to_string())
                    .or_insert_with(Vec::new)
                    .push(voltage);
            }
        }
    }

    let mut result = Vec::new();

    for (satellite_id, voltages) in data {
        let min = voltages.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = voltages.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let sum: f32 = voltages.iter().sum();
        let avg = sum / voltages.len() as f32;
        result.push(format!(
            "{}: min={:.1}, max={:.1}, avg={:.1}",
            satellite_id, min, max, avg
        ));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_battery_logs() {
        let input = vec![
            "2025-09-17T12:00Z SAT-1 VOLT=3.7",
            "2025-09-17T12:01Z SAT-1 VOLT=3.6",
            "2025-09-17T12:02Z SAT-2 VOLT=3.8",
            "2025-09-17T12:03Z SAT-1 VOLT=3.5",
        ];

        let expected = vec![
            "SAT-1: min=3.5, max=3.7, avg=3.6",
            "SAT-2: min=3.8, max=3.8, avg=3.8",
        ];

        let result = analyze_battery_logs(&input);
        assert_eq!(result, expected);
    }
}    


/*
initialize empty map satellite_data: Map<String, Vec<f32>>

for each line in input_lines:
    split the line by spaces
    satellite_id = parts[1]
    voltage_str = parts[2].split("=")[1]
    voltage = parse voltage_str as float

    append voltage to satellite_data[satellite_id]

for each satellite_id in sorted keys of satellite_data:
    voltages = satellite_data[satellite_id]
    min = minimum(voltages)
    max = maximum(voltages)
    avg = sum(voltages) / count(voltages)

    print "{satellite_id}: min={min:.1f}, max={max:.1f}, avg={avg:.1f}"
*/