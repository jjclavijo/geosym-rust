use std::fs;
use serde::Deserialize;
use std::path::Path;
use std::io;

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
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Ggm {
    pub time_noise_start: i32,
    pub m: i32,
    pub sigma: f32,
    pub kappa: f32,
    pub one_minus_phi: f32,
    pub dt: f32, 
    pub units: String
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Linear {
    pub time_noise_start: i32,
    pub m: i32,
    pub sigma: f32,
    pub kappa: f32,
    pub one_minus_phi: f32,
    pub dt: f32, 
    pub units: String
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct NoiseModels {
    pub ggm: Option<Ggm>,
    pub linear: Option<Linear>
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Config {
    pub general: General,
    pub noise_models: NoiseModels,
}

pub fn load_config_toml(filename: &Path) -> Result<Config, io::Error> {
    let content = fs::read_to_string(filename)?; 
    let value: Config = toml::from_str(&content).expect("Error parsing file");
    Ok(value)
}

#[cfg(test)]
mod tests {
    use crate::utils::load_config_toml;
    use std::path::Path;

    #[test]
    fn test_load_config_toml_ok() {
        let path = Path::new("./test2.toml");
        let config = load_config_toml(&path).unwrap();
        println!("{:?}", config);

        match config.noise_models.ggm {
            Some(x) => assert_eq!(x.m, 1000),
            None => panic!("Missing ggm"),
        }
        
        match config.noise_models.linear {
            Some(x) => assert_eq!(x.m, 1000),
            None => panic!("Missing linear"),
        }
    }
}
