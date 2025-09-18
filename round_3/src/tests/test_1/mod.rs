use std::fs::File;
use std::io::{self, BufRead};

/*
    Return minutes until battery first reaches 0, or -1 if it never depletes.
    a: solar generation in W during sun (first 60 min of each 120-min cycle)
    b: constant consumption in W
    c: initial energy in Wh
*/
pub fn minutes_until_depletion(a: i64, b: i64, c: i64) -> i64 {
    // No consumption - never depletes
    if b == 0 { return -1; }

        // HElper: integer ceil division for non-negative numerator and positive denom
        fn ceil_div_nonneg(x: i64, d: i64) -> i64 { if x <= 0 { 0 } else { (x + d - 1) / d } }


        // Phase 1: Sun (first 60 minutes starting at t=0)
        if a < b {
        // Net drain rate (W) during sun.
        let drain = b - a; // > 0
        // Minutes to hit zero within sun (floor, truncation)
        let t_sun = (60 * c) / drain; // floor(c / (drain/60))
        if t_sun < 60 { return t_sun; }
        // else continue after full sun
    }


    // ENergy after the first sun hour.
    let e_after_sun = c + (a - b);


    // If battery already empty exatly at end of sun
    if e_after_sun <= 0 { return 60; }


    // Phase 2: First shadow (next 60 minutes)
    // If it depletes during this shadow
    if e_after_sun <= b { // hits zero within this shadow (<= includes exact end)
    let t_shadow = (60 * e_after_sun) / b; // minutes within shadow
        return 60 + t_shadow;
    }


    // Phase 3: Multi-cycle analysis
    // Net change per full 120-min cycle: a - 2b (Wh)
    let net_per_cycle = a - 2 * b; // could be negative, zero, or positive


    // If cycles are non-decreasing or flat, and we survived first shadow => never depletes
    if net_per_cycle >= 0 { return -1; }


    // We will eventually deplete in some later shadow.
    // Let d = -(net_per_cycle) = 2b - a (> 0)
    let d = -net_per_cycle; // > 0


    // After n full cycles (n >= 1), energy is: E_n = c - n*d
    // Before shadow of cycle n (0-indexed cycles), energy is: E_n + (a - b)
    // We need smallest n >= 0 such that: E_n + (a - b) <= b
    // c - n*d + (a - b) <= b => c + a - 2b <= n*d
    // n >= ceil( (c + a - 2b) / d ), clamp to >= 0
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
        assert_eq!(minutes_until_depletion(30, 25, 200), 1320);
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