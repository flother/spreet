use clap::Parser;
use walkdir::WalkDir;

mod cli;
mod fs;

fn main() {
    let args = cli::Cli::parse();
    println!("Retina: {}", args.retina);
    println!("Ratio: {}", args.ratio);
    println!("Input: {:?}", args.input);
    println!("Output: {}", args.output);

    let walker = WalkDir::new(args.input).follow_links(true).into_iter();
    for entry in walker.filter_entry(|e| fs::is_interesting_input(e)) {
        println!("{}", entry.unwrap().path().display());
    }
}
