use rand_distr::{Normal};
use rand_chacha::ChaCha8Rng;
use rand::{Rng,SeedableRng};

use std::fs::File;
use std::io::prelude::*;

use byteorder::{ByteOrder, LittleEndian};

fn main() -> std::io::Result<()> {
    const LARGO:usize = 10000;
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let normal = Normal::new(0.0,1.0).unwrap();
    let vals: Vec<f32> = (0..LARGO).map(|_| rng.sample(&normal)).collect();

    // println!("{:?}",vals) ;

    {
        let mut file = File::create("test")?;
        let mut bytes = [0;LARGO * 4];
        LittleEndian::write_f32_into(&vals, &mut bytes);
        // Write a slice of bytes to the file
        //

        file.write_all(&bytes)?;
    }

    {
        let mut file = File::open("test")?;
        // read the same file back into a Vec of bytes
        let mut buffer = Vec::<u8>::new();
        let mut valores = [0.0;LARGO];
        file.read_to_end(&mut buffer)?;
        LittleEndian::read_f32_into(&buffer, &mut valores);

        //println!("{:?}", valores);
        assert_eq!(vals, valores);
    }

    Ok(())
}
