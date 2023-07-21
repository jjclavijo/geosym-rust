use crate::utils::Config;
//use crate::utils::NoiseModels;
use std::time::Instant;
use rand::thread_rng;
//use rand::rngs::ThreadRng;

pub fn simulate_noise(config: &Config) -> bool {
        
    //--- Some variables that define the runs
    // Unpack all variables from the control object
    let n_simulations = config.general.number_of_simulations;
    let m = config.general.number_of_points;
    let dt = config.general.sampling_period;
    let ms = config.general.time_noise_start;
    //let noise_models = &config.noise_models;

    let repeatable_noise = config.general.repeatable_noise;
    let deterministic_noise = config.general.deterministic_noise;
    
    // Start timer
    let start_time = Instant::now();
    
    // Create random number generator
    let rng = thread_rng();

    // Run all simulations
    let mut simulations: Vec<Vec<f32>> = vec![];
    
    //for k in 0..n_simulations {
    //    let y = create_noise(m, dt, ms, noise_models, &rng);
    //    simulations.push(y)
    //}

    let elapsed = start_time.elapsed();
    println!("--- {:} seconds ---", elapsed.as_secs());
    
    for s in simulations.iter() {
        println!("{:?}", s);
    }
    true
}



//fn create_noise(m: i32, dt: f32, ms: i32, noise_models: &NoiseModels, rng: &ThreadRng) -> Vec<f32> {
//
//    println!("create_noise parameters {:?} {:?} {:?}", m, dt, ms);
//    let sigma: Vec<f32> = Vec::new();
//    let h: Vec<f32> = Vec::new();
//    
//    //for model in noise_models.items() {
//    //    println!("{:?}", model);
//    //}
//
//    sigma
//}


//def create_noise(
//        m,
//        dt,
//        ms,
//        noiseModels,
//        rng=None
//    ):
//    """ Toma la lista de los modelos que quiere usar junto con los parametros y
//    instancia los nuevos modelos desde create_h
//    """
//    print("Params de create_noise: ", m, dt, ms, noiseModels, rng)
//    sigma = []
//    h = []
//    for model, params in noiseModels.items():
//        ## Dinamycally get and call the function using the model name
//        model_function = getattr(create_hs, model)
//
//        for i in params.keys():
//            ## Cada modelo puede estar mas de una vez
//            print(f"--> About to run model {model} with params {params[i]}")
//            single_s, single_h = model_function(**params[i])
//            sigma.append(single_s)
//            h.append(single_h)
//
//    #--- Create random number generator
//    if rng is None:
//        rng = np.random.default_rng()
//
//    #--- Create the synthetic noise
//    y = np.zeros(m)
//
//    for s,ha in zip(sigma,h):
//        w = s * rng.standard_normal(m+ms)
//        y += signal.fftconvolve(ha, w)[0:m]
//
//    return y


#[cfg(test)]
mod tests {
    #[test]
    fn test_simulate_noise() {
        assert_eq!(1, 1);
    }
}
