#[macro_use]
extern crate log;
extern crate env_logger;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate quackin;
extern crate failure;

use structopt::StructOpt;
use quackin::build_quackin;
use failure::Error;
use log::Level;

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
    split: u32,
    /// Recommender method to use.
    #[structopt(short = "r", long = "reco", default_value = "knnuser")]
    reco: String,
}

fn quackin(args: Args) -> Result<(), Error> {
    let quackin = build_quackin(
        args.input,
        args.evaluator,
        args.split,
        args.reco,
    )?;

    info!("Building model started");
    quackin.build_model();

    info!("Evaluation");
    quackin.evaluate();

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