#[derive(Debug)]
pub struct SatData {
    pub name: String,
    // TODO: Make parameter handling generic
    pub latest_temp: f64,
    pub temp_sum: f64,
    pub temp_count: f64,
    pub volt: f64,
}

impl SatData {
    pub fn new(name: &str) -> Self {
        SatData {
            name: name.to_string(),
            latest_temp: 0.0,
            temp_sum: 0.0,
            temp_count: 0.0,
            volt: 0.0,
        }
    }

    pub fn register_param(&mut self, param_name: &str, value: &str) {
        match value.parse::<f64>() {
            Ok(val) => match param_name {
                "TEMP" => self.register_temp(val),
                "VOLT" => self.register_volt(val),
                _ => eprintln!("Unknown parameter: {}", param_name),
            },
            Err(_) => {
                eprintln!("Could not parse value '{}' for param '{}'", value, param_name);
            }
        }
    }

    pub fn register_temp(&mut self, temp: f64) {
        self.latest_temp = temp;
        self.temp_sum += temp;
        self.temp_count += 1.0;

        get_temp_average(&self);
    }

    pub fn register_volt(&mut self, volt: f64) {
        self.volt = volt;
    }

    pub fn get_temp_average(&self) -> f64 {
        if self.temp_count == 0.0 {
            0.0
        } else {
            self.temp_sum / self.temp_count
        }
    }
}