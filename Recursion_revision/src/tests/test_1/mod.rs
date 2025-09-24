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

struct Cycle {
    phases: Vec<Phase>
}

pub fn minutes_until_depletion(a: i64, b: i64, c: i64) -> i64 {
    // No consumption - never depletes
    if b == 0 { return -1; }

    let mut state = EnergyState { reserve: starting_energy, time: 0 };
    for phase in phases.iter().cycle() {
        let energy_gained = phase.gain_rate * phase.duration_minutes / 60;
        let energy_drained = phase.drain_rate * phase.duration_minutes / 60;

        if state.reserve + energy_gained < energy_drained {
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
    return -1;
}