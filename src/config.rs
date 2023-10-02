use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize)]
pub enum Unit {
    Celsius,
    Fahrenheit,
    Kelvin,
}


/// This struct contains all possible configuration fields. 
/// It should not be used as mutable; all data in this struct should effectively be treated as read-only.
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub units: Unit,
}

impl Default for Config {
    fn default() -> Config {
        Config { 
            api_key: "APIKEY".to_owned(), 
            units: Unit::Kelvin 
        }
    }
}