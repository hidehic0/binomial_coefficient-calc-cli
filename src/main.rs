mod calc_binomial_coefficient;
mod factorial;

use clap::Parser;
use std::process;

use crate::calc_binomial_coefficient::get_binomial_coefficient;

// use crate::factorial::getfactorial;

#[derive(Parser)]
#[clap(name = env!("CARGO_PKG_NAME"),version = env!("CARGO_PKG_VERSION"),arg_required_else_help = true,)]
struct Args {
    n: u64,
    r: u64,
}

fn main() {
    let args = Args::parse();

    if args.n < args.r {
        println!("0");
        process::exit(0);
    }

    if args.n > 10_u64.pow(4) * 5 {
        println!("n is too big");
        process::exit(1);
    }

    println!("{}", get_binomial_coefficient(args.n, args.r));
}
