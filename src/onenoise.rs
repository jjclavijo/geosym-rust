use crate::utils::Config;
//use crate::utils::NoiseModels;
use std::time::Instant;
use rand::Rng;
//use rand::rngs::ThreadRng;
use rand_distr::Normal;
use crate::fftconvolve::fftconvolve_vec;
use crate::create_hs::powerlaw;

fn create_noise(impulso: SigmaH, rng: Rng) -> Vec<f32> {
    let SigmaH { sigma, h } = impulso;
    let normal = Normal<f32>;
    let m = h.len()

    let vals: Vec<f32> = (0..m).map(|_| rng.sample(&normal)).collect();

    fftconvolve(h,vals)[0..m].to_vec()
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
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_random_numbers(){
        let mut rng = ChaCha8Rng::fromSeed(42)

        let normal = Normal<f32>;
        let vals: Vec<f32> = (0..1000).map(|_| rng.sample(&normal)).collect();

        println!("{:?}",vals) 
    }

    // #[test]
    // fn test_power_law_ok(){
    //     //create_hs.Powerlaw(m=20, kappa=-1, sigma=2, units="mom", dt=0.5)
    //     let m: usize = 20;
    //     let kappa: f32 = -1.0;
    //     let sigma: f32 = 2.0;
    //     let dt: f32 = 0.5;

    //     let expected_number: f32 = 0.3847024397698062;
    //     let expected_array: Vec<f32> = vec![1. , 0.5, 0.375, 0.3125, 0.2734375 , 0.24609375, 0.22558594, 0.20947266, 0.19638062, 0.18547058, 0.17619705, 0.1681881 , 0.16118026, 0.15498102, 0.14944598, 0.14446445, 0.13994993, 0.13583376, 0.1320606 , 0.12858532];
    //     
    //     let response = powerlaw(m, kappa, sigma, Units::mom, dt).unwrap();
    //     assert_eq!(response.gmsv, expected_number);
    //     assert_eq_float_vec!(response.rpf.h, expected_array, 0.001);
    // }
}
