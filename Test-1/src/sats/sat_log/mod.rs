use std::collections::HashMap;
use crate::domain::SatData;

pub struct TelemetryLogEntry {
    pub timestamp: String,
    pub sat_name: String,
    pub param: String,
    pub value: f64,
}

// Global state (or passed into the function)
let mut sat_dict: HashMap<String, SatData> = HashMap::new();
let mut telemetry_log: Vec<TelemetryLogEntry> = Vec::new();



/*
// HashMap of sat_dict.
//sat_dict

// Vector of Telemetry
// telemetry_log

// register_sat_telemetry
pub fn register_sat_telemetry(telemetry_entry:&str){ // i.e. 2025-09-17T11:33Z SAT-1 VOLT=3.5
    // Parse the telemetry_entry
    //2025-09-17T11:33Z SAT-1 VOLT=3.5

    // assert timestamp 
    // timestamp = parsed_timestamp

    // assert name, TODO check existing list of sats.
    // sat_name = name;

    // Parse param_name and value;

    // Get sat_data with key: sat_name from sat_dict
    // If sat_data doesn't create and add it to SatData.

    // sat_data.register_param(param_name, value_string) // TODO pass full querystring, with differents parameters.
    //

    // Add to the vector telemetry_log 
    // {timestamp, sat_data.clone() }
}
    */