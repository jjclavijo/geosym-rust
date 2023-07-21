use std::fs;
use serde::Deserialize;
use std::path::Path;
use std::io;
use std::collections::HashMap;

//TODO: Add toher models configuration
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct General {
    pub number_of_points: i32,
    pub sampling_period: f32,
    pub time_noise_start: i32,
    pub number_of_simulations: i32,
    pub repeatable_noise: bool,
    pub deterministic_noise: bool,
    pub order: Vec<ModelEnum>
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Copy, Clone)]
pub enum ModelEnum {
    Ggm,
    Linear
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Copy, Clone)]
pub enum Units {
    mom,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Ggm {
    pub time_noise_start: i32,
    pub m: i32,
    pub sigma: f32,
    pub kappa: f32,
    pub one_minus_phi: f32,
    pub dt: f32, 
    pub units: Units
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Linear {
    pub time_noise_start: i32,
    pub m: i32,
    pub sigma: f32,
    pub kappa: f32,
    pub one_minus_phi: f32,
    pub dt: f32, 
    pub units: Units
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub enum Model {
    Ggm(Ggm),
    Linear(Linear),
}

impl Config {
    pub fn get_model(&self, model: &str) -> Result<Model, String> {
        // Get a model using the name string
        match model {
            "GGM" => Ok(Model::Ggm(self.ggm.unwrap().clone())),
            "Linear" => Ok(Model::Linear(self.linear.unwrap().clone())),
            _ => Err("Model not found".to_string()),
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Config {
    pub general: General,
    pub ggm: Option<Ggm>,
    pub linear: Option<Linear>
}

pub fn load_config_toml(filename: &Path) -> Result<Config, io::Error> {
    let content = fs::read_to_string(filename)?; 
    let value: Config = toml::from_str(&content).expect("Error parsing file");
    Ok(value)
}

#[cfg(test)]
mod tests {
    use crate::utils::{load_config_toml, Units};
    use std::path::Path;
    
    #[test]
    fn test_get_function_list_ok() {
        let path = Path::new("./test2.toml");
        let config = load_config_toml(&path).unwrap();
        println!("{:#?}", config);
        println!("{:#?}", config.get_model("GGM").unwrap());
    }

    #[test]
    fn test_load_config_toml_ok() {
        let path = Path::new("./test2.toml");
        let config = load_config_toml(&path).unwrap();
        println!("{:#?}", config);

        match config.ggm {
            Some(x) => assert_eq!(x.m, 1000),
            None => panic!("Missing ggm"),
        }

        match config.linear {
            Some(x) => assert_eq!(x.m, 1000),
            None => panic!("Missing linear"),
        }
    }
}
