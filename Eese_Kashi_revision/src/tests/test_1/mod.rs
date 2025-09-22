use std::fs::File;
use std::io::{self, BufRead};

/// Returns the minutes until depletation for a given phase minutes duration
fn get_phase_minutes_until_depletion(energy_reserve: i64, sun_gain: i64, consumption_per_hour: i64, phase_minutes:i64) -> i64{
    let net_drain_per_hour = consumption_per_hour - sun_gain;
    let minutes_until_depletion = (phase_minutes * energy_reserve) / net_drain_per_hour;
    return minutes_until_depletion;
}

/// Returns true if the battery can survive the full phase_minutes.
fn get_survives_phase(energy_reserve: i64, sun_gain: i64, consumption_per_hour: i64, phase_minutes:i64) -> bool {
    let minutes_until_depletion = get_phase_minutes_until_depletion(energy_reserve: i64, sun_gain: i64, consumption_per_hour: i64, phase_minutes:i64)
    return minutes_until_depletion >= phase_minutes;
}


/*
    Return minutes until battery first reaches 0, or -1 if it never depletes.
    a: sun_gain
    b: consumption_per_hour
    c: energy_reserve
*/
pub fn minutes_until_depletion(
    sun_gain: i64, 
    consumption_per_hour: i64,
    energy_reserve: i64
) -> i64 {

    /// Returned when the satellite never depletes battery.
    let ETERNAL:i64 = -1;
    let SUN_PHASE_MINUTES  = 60;
    let DARK_PHASE_MINUTES = 60;

    // Case 1 - Never depletes
    // No consumption - never depletes
    if consumption_per_hour == 0 { 
        // Early return
        return ETERNAL;
    }

    // Helper: integer ceil division for non-negative numerator and positive denom
    fn ceil_div_nonneg(x: i64, d: i64) -> i64 { if x <= 0 { 0 } else { (x + d - 1) / d } }


    // Phase 1: Sun (first 60 minutes starting at t=0)
    if sun_gain < consumption_per_hour {
        // Net drain rate (W) during sun.
        let drain = consumption_per_hour - sun_gain; // > 0

        // Minutes to hit zero within sun (floor, truncation)
        let minutes_to_zero_sun = (SUN_PHASE_MINUTES * energy_reserve) / drain; // drain_per_minute = drain/60 | floor(energy_reserve / drain_per_minute)
        if minutes_to_zero_sun < SUN_PHASE_MINUTES { 

            //Case 2 - Depletion in sunlight
            // Early return
            return minutes_to_zero_sun;
        }
        // else continue after full sun
    }

    // Now we walk into the dark
    // Energy after the first sun hour.
    let energy_at_dark_phase = energy_reserve + (sun_gain - consumption_per_hour);


    // If battery already empty exatly at end of sun
    if energy_at_dark_phase <= 0 {
        //Case 3 - Depletion in the dark gate
        // Early return
        return SUN_PHASE_MINUTES;
    }

    // Phase 2: First shadow (next 60 minutes)
    // If it depletes during this shadow
    if energy_at_dark_phase <= consumption_per_hour { // hits zero within this shadow (<= includes exact end)
        /*
            consumption_per_hour______ DARK_PHASE_MINUTES
            energy_at_dark_phase _____ minutes_to_zero_dark
        */
        let minutes_to_zero_dark = (DARK_PHASE_MINUTES * energy_at_dark_phase) / consumption_per_hour; // minutes within shadow
        return SUN_PHASE_MINUTES + minutes_to_zero_dark;
    }


    // Phase 3: Multi-cycle analysis
    // Net change per full 120-min cycle: a - 2b (Wh)
    let consumption_per_cycle.   = consumption_per_hour * 2
    let energy_reserve_per_cycle = sun_gain - consumption_per_cycle; // could be negative, zero, or positive

    // If cycles are non-decreasing or flat, and we survived first shadow => never depletes
    if energy_reserve_per_cycle >= 0 { return ETERNAL; }

    
    // We will eventually deplete in some later shadow.
    // Let d = -(energy_reserve_per_cycle) = 2b - a (> 0)
    let energy_loss_per_cycle = -energy_reserve_per_cycle; // > 0

    /*
        After full_cycles (full_cycles >= 1), 
        full_cycles_energy_reserve = energy_reserve - ( full_cycles * energy_loss_per_cycle )

        Then—before entering the shadow of the next cycle:
        We still get the 60 minutes of sunlight. So the battery gets one last sunlight boost:
        let energy_before_final_shadow = full_cycles_energy_reserve + (sun_gain - consumption_per_hour);
        
        We need smallest n >= 0 such that: E_n + (a - b) <= b
        c - n*d + (a - b) <= b => c + a - 2b <= n*d
        n >= ceil( (c + a - 2b) / d ), clamp to >= 0
    */
    let need = c + a - 2 * b; // could be <= 0 (then n = 0)
    let n = ceil_div_nonneg(need, d); // number of full cycles before final shadow


    let e_before_shadow = c - n * d + (a - b); // energy entering the final shadow
    debug_assert!(e_before_shadow <= b);


    let minutes_in_final_shadow = (60 * e_before_shadow) / b; // floor


    // Total minutes: n full cycles * 120 + first sun (60) + minutes into shadow
    120 * n + 60 + minutes_in_final_shadow
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() { // 40 30 20 -> 120
        assert_eq!(minutes_until_depletion(40, 30, 20), 120);
    }

    #[test]
    fn sample2() { // 40 10 5 -> -1
        assert_eq!(minutes_until_depletion(40, 10, 5), -1);
    }

    #[test]
    fn deplete_in_first_sun() { // a<b, depletes within sun
        // a=10, b=40, c=20 => 20 Wh / (30 W) = 0.666.. h => 40 min
        assert_eq!(minutes_until_depletion(10, 40, 20), 40);
    }

    #[test]
    fn exact_end_of_first_shadow() { // equal at end of shadow
        // a=30, b=20, c=10: after sun E=20, shadow drain b=20 => hit at 60+60=120
        assert_eq!(minutes_until_depletion(30, 20, 10), 120);
    }

    #[test]
    fn later_cycle_shadow() {
        // Choose values where a>=b, net_per_cycle negative, survive first shadow
        // a=30, b=25, c=200
        // e_after_sun=205? wait c+(a-b)=200+5=205; e_after_sun/b=205/25>1 => survive
        // d=2b-a=50-30=20. need=c+a-2b=200+30-50=180. n=ceil(180/20)=9 cycles
        // e_before_shadow=c - n*d + (a-b) = 200 - 180 + 5 = 25 => equals b
        // time = 9*120 + 60 + 60 = 1320
         assert_eq!(minutes_until_depletion(30, 25, 200), 1200);
    }
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