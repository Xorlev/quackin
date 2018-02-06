#[macro_use]
extern crate log;
extern crate env_logger;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate quackin;
//use std::fs::File;
//use std::io::prelude::*;

//use log::Level;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Args {
    /// Input data file name.
    #[structopt(short = "i", long = "input")]
    input: String,
    /// evaluator name.
    #[structopt(short = "e", long = "evaluator")]
    evaluator: String,
    /// Split Train/Test in percents.
    #[structopt(short = "s", long = "split", default_value = "80")]
    print_stats: u32,
}

fn quackin(args: Args) -> Result<(), Error> {
    let quackin = build_quackin(
        args.input,
        !args.disable_geom,
        args.libpostal_path.into(),
        args.country_code,
    )?;
    
    Ok(())
}

fn main() {
    env_logger::init();
    let args = Args::from_args();
    info!("Quackin launching...");

    match quackin(args) {
        Err(e) => {
            error!("quackin in error! {:?}", e);
            e.causes().for_each(|c| {
                error!("{}", c);
                if let Some(b) = c.backtrace() {
                    error!("  - {}", b);
                }
            });

            std::process::exit(1);
        }
        _ => (),
    }    
}

