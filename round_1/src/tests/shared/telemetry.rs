use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SatData {
    pub latest_params: HashMap<String, f64>,
    pub temp_sum: f64,
    pub temp_count: u32,
}

impl SatData {
    pub fn new() -> Self {
        SatData {
            latest_params: HashMap::new(),
            temp_sum: 0.0,
            temp_count: 0,
        }
    }

    pub fn register_param(&mut self, param: &str, value: f64) {
        if param == "TEMP" {
            self.temp_sum += value;
            self.temp_count += 1;
        }
        self.latest_params.insert(param.to_string(), value);
    }

    pub fn get_temp_avg(&self) -> Option<f64> {
        if self.temp_count == 0 {
            None
        } else {
            Some(self.temp_sum / self.temp_count as f64)
        }
    }
}
