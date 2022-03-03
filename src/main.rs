use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[clap(version, about)]
#[clap(group(ArgGroup::new("pixel_ratio").args(&["ratio", "retina"])))]
struct Cli {
    /// A directory of SVGs to include in the spritesheet
    input: String,
    /// Name of the file in which to save the spritesheet
    output: String,
    /// Set the output pixel ratio
    #[clap(short, long, default_value_t = 1)]
    ratio: u8,
    /// Set the pixel ratio to 2 (equivalent to --ratio=2)
    #[clap(long)]
    retina: bool,
}
fn main() {
    let cli = Cli::parse();
    println!("Retina: {}", cli.retina);
    println!("Ratio: {}", cli.ratio);
    println!("Input: {}", cli.input);
    println!("Output: {}", cli.output);
}
