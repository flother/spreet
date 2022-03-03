use clap::Parser;

mod cli;

fn main() {
    let args = cli::Cli::parse();
    println!("Retina: {}", args.retina);
    println!("Ratio: {}", args.ratio);
    println!("Input: {}", args.input);
    println!("Output: {}", args.output);
}
