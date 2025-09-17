use std::collections::HashMap;
use crate::tests::shared::telemetry::SatData;

pub fn run_telemetry_parser() {
    let logs = vec![
        "2025-09-17T11:30Z SAT-3 TEMP=23.5",
        "2025-09-17T11:31Z SAT-3 VOLT=3.7",
        "2025-09-17T11:32Z SAT-1 TEMP=19.1",
        "2025-09-17T11:33Z SAT-3 TEMP=24.0",
        "2025-09-17T11:33Z SAT-1 VOLT=3.5",
    ];

    let mut sat_map: HashMap<String, SatData> = HashMap::new();

    for line in logs {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 3 { continue; }

        let sat_id = parts[1];
        let param_parts: Vec<&str> = parts[2].split('=').collect();
        if param_parts.len() != 2 { continue; }

        let param = param_parts[0];
        let value: f64 = param_parts[1].parse().unwrap_or(0.0);

        let sat_data = sat_map.entry(sat_id.to_string()).or_insert(SatData::new());
        sat_data.register_param(param, value);
    }

    // Sort and print
    let mut keys: Vec<&String> = sat_map.keys().collect();
    keys.sort();

    for sat in keys {
        println!("{}:", sat);
        if let Some(data) = sat_map.get(sat) {
            for (param, value) in &data.latest_params {
                if param == "TEMP" {
                    let avg = data.get_temp_avg().unwrap_or(0.0);
                    println!("  TEMP: latest={}, avg={}", value, avg);
                } else {
                    println!("  {}: {}", param, value);
                }
            }
        }
    }
}