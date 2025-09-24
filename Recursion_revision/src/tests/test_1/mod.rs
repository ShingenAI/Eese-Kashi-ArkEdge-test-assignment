use std::fs::File;
use std::io::{self, BufRead};

// The current state of the system at any point in time
struct EnergyState {
    reserve: i64, // battery
    time: i64,    // minutes passed
}

// What each phase does to the energy system
struct Phase {
    duration_minutes: i64,
    gain_rate: i64,   // a (in W)
    drain_rate: i64,  // b (in W)
}

pub fn minutes_until_depletion(energy_gained: i64, energy_drained: i64, starting_energy: i64) -> i64 {
    // No consumption - never depletes
    if energy_drained == 0  {
        // Early return
        // Is Eternal
        return -1; 
    }

    let phases = vec![
        Phase {
            duration_minutes: 60,
            gain_rate: energy_gained,
            drain_rate: energy_drained,
        },
        Phase {
            duration_minutes: 60,
            gain_rate: 0,
            drain_rate: energy_drained,
        },
    ];

    let net_gain = phases.iter().map(|p| {
        (p.gain_rate - p.drain_rate) * p.duration_minutes / 60
    }).sum::<i64>();

    if net_gain >= 0 {
        // Early return
        // Is Eternal
        return -1;
    }

    let mut state = EnergyState { reserve: starting_energy, time: 0 };
    
    for phase in phases.iter().cycle() {
        let energy_gained = phase.gain_rate * phase.duration_minutes / 60;
        let energy_drained = phase.drain_rate * phase.duration_minutes / 60;

        if state.reserve + energy_gained <= energy_drained {
            // Death happens within this phase — compute exact minute
            let drain_per_minute = phase.drain_rate - phase.gain_rate;
            let minutes = (60 * state.reserve) / drain_per_minute;
            return state.time + minutes;
        }

        // Apply full phase
        state.reserve += energy_gained - energy_drained;
        state.time += phase.duration_minutes;
    }

    // If loop never ends, return -1 (ETERNAL)
    -1
}

pub fn get_minutes_until_depletion(path: &str) -> io::Result<i64> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue; // skip empty lines
        }

        let values: Vec<i64> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap_or(0))
            .collect();

        if values.len() == 3 {
            return Ok(minutes_until_depletion(values[0], values[1], values[2]));
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Expected 3 values"));
        }
    }

    Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No valid lines found"))
}