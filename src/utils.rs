use std::fs;
use serde::Deserialize;
use std::path::Path;

//TODO: Add all toher models configuration

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct General {
    number_of_points: i32,
    sampling_period: f32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Params {
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
pub struct Ggm {
    pub params: Params
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Linear {
    pub params: Params
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



pub fn load_config_toml(filename: &Path) -> Result<Config, &str> {

    let content = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => return Err("Error abriendo archivo"),
    };

    let value: Config = match toml::from_str(&content) {
        Ok(v) => v,
        Err(e) => return Err("error parseando archivo"),
    };
    Ok(value)
}

#[cfg(test)]
mod tests {
    use crate::utils::load_config_toml;
    use std::path::Path;

    #[test]
    fn test_load_config_toml_ok() {
        let path = Path::new("./test.toml");
        let config = load_config_toml(&path).unwrap();

        match config.noise_models.ggm {
            Some(x) => assert_eq!(x.params.m, 1000),
            None => panic!("Missing ggm"),
        }
        
        match config.noise_models.linear {
            Some(x) => assert_eq!(x.params.m, 1000),
            None => panic!("Missing linear"),
        }
    }
}
