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

    pub fn registerTemp(&mut self, temp:f64){
        self.latest_temp = temp;
        self.temp_count += 1;
        self.temp_sum += temp;
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