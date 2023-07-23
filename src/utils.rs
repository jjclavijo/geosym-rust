use std::fs;
use serde::Deserialize;
use std::path::Path;
use std::io;
//use crate::create_hs::white_noise;

#[allow(dead_code)]
#[derive(Deserialize, Debug, Copy, Clone)]
pub enum Units {
    mom,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Config {
    pub general: General,
    pub ggm: Ggm,
    pub linear: Linear,
}

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
    pub order: Vec<String>
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

#[derive(Debug)]
enum Model {
    GGM(Ggm),
    LINEAR(Linear),
}

fn culo_negro(i: i32, b: i32) -> Vec<i32>{
    println!("Culo negro");
    let v: Vec<i32> = vec![1, 5];
    v
}

fn culo_gris(i: i32, b: i32) -> Vec<i32>{
    println!("Culo negro");
    let v: Vec<i32> = vec![1, 5];
    v
}

impl Model {
    fn call(&self) -> Vec<i32> {
        let response = match &self {
            Model::GGM(x) => culo_negro(x.m, x.m),
            Model::LINEAR(x) => culo_gris(x.m, x.m),
        };
        response
    }
}

fn model_list_from_config(config: &Config) -> Vec<Model> {
    let mut res: Vec<Model> = Vec::new();
    for model in config.general.order.iter() {
        match model.as_str() {
            "GGM" => res.push(Model::GGM(config.ggm)),
            "LINEAR" => res.push(Model::LINEAR(config.linear)),
            _ => panic!("No model name")
        }
    }
    res
}

pub fn load_config_toml(filename: &Path) -> Result<Config, io::Error> {
    let content = fs::read_to_string(filename)?; 
    let config: Config = toml::from_str(&content).expect("Error parsing file");
    let lista = model_list_from_config(&config);

    println!("---> {:#?}", lista);
    Ok(config)
}


#[cfg(test)]
mod tests {
    use crate::utils::{load_config_toml, Units, model_list_from_config};
    use std::path::Path;
    
    #[test]
    fn test_utils_get_function_list_ok() {
        let path = Path::new("./test2.toml");
        let config = load_config_toml(&path).unwrap();
        println!("{:#?}", config);

        let lista = model_list_from_config(&config);
        println!("---> {:#?}", lista);

        let mut res: Vec<Vec<i32>> = Vec::new();
        for i in lista.iter() {
            res.push(i.call())
        }

        println!("{:?}", res);
        assert_eq!(1,2);
    }

    //#[test]
    //fn test_utils_load_config_toml_ok() {
    //    let path = Path::new("./test2.toml");
    //    let config = load_config_toml(&path).unwrap();
    //    println!("{:#?}", config);

    //    match config.ggm {
    //        Some(x) => assert_eq!(x.m, 1000),
    //        None => panic!("Missing ggm"),
    //    }

    //    match config.linear {
    //        Some(x) => assert_eq!(x.m, 1000),
    //        None => panic!("Missing linear"),
    //    }
    //}
}
