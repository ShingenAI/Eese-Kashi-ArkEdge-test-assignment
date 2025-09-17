#[derive(Debug)]
pub struct SatData {
    pub name:String,
    // TODO: Make parameter handling generic
    pub latest_temp: f64,
    pub temp_sum: i64,
    pub temp_count: f64,
    pub volt: f64,
}

impl SatData {
    pub fn new (
        name:&str
    ) -> Self {

        SatData {
            name: name.to_string(),
            temp_count: 0
        }       
    }

    pub fn register_param(&mut self, param_name: &str, value: f64) {
        match param_name {
            "TEMP" => self.register_temp(value),
            "VOLT" => self.register_volt(value),
            _ => {
                // TODO: Handle new params
                eprintln!("Unknown param: {}", param_name);
            }
        }
    }

    pub fn registerTemp(&mut self, temp:f64){
        self.latest_temp = temp;
        self.temp_count += 1;
        self.temp_sum += temp;
    }

    pub fn register_volt(&mut self, volt:f64){
        self.volt = temp;
    }

    pub fun get_temp_average(&self) -> f64 {
        // TODO Track TEMP with sum/count
        if self.temp_count == 0 {
            0.0
        } else {
            self.temp_sum / self.temp_count as f64
        }
    }
}