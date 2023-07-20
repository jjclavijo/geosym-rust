use std::io;
use std::error::Error;

pub struct SigmaH {
    pub sigma: f32,
    pub h: Vec<f32>,
}

impl SigmaH {
    fn new(sigma: f32, h: Vec<f32>) -> SigmaH {
        SigmaH {
            sigma,
            h,
        }
    }
}

pub fn white_noise(m: usize, sigma: f32) -> Result<SigmaH, Box<dyn Error>> {
    // Create impulse function for White noise.
    let mut h: Vec<f32> = vec![0.0; m];
    h[0] = sigma;
    Ok(SigmaH::new(sigma, h))
}




#[cfg(test)]
mod tests {
    use crate::create_hs::white_noise;

    #[test]
    fn test_white_ok() {
        let sigma = 1.0;
        let m: usize = 10;
        let sigma_h = white_noise(m, sigma).unwrap();
        assert_eq!(sigma_h.sigma, sigma);
        assert_eq!(sigma_h.h, vec![sigma, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }
}
