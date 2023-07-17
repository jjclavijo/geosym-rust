mod utils;
mod simulatenoise;
use clap::Parser;
use crate::utils::Config;

#[derive(Parser)]
struct Cli {
    control_file: std::path::PathBuf,
}

fn main() {
    
    println!("***************************************");
    println!("    simulatenoise, version 0.1.1");
    println!("***************************************");

    let args = Cli::parse();
    println!("{:?}", args.control_file);
    
    let config: Config = utils::load_config_toml(args.control_file.as_path()).unwrap();
    println!("{:?}", config.general);

    // let noise = simulatenoise::simulate_noise(&config);
    // println!("{:?}", noise);

}
//
//    simulaciones = simulate_noise(control)
//
//    for simulacion in simulaciones:
//        print( '\n'.join(
//            map('{:.3f}'.format,
//                simulacion)
//                        )
//              )
