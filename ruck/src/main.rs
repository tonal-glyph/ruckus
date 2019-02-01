use quicli::prelude::*;
use structopt::StructOpt;
// Add cool slogan for your app here, e.g.:
/// Get first n lines of a file
#[derive(Debug, StructOpt)]
struct Cli {
    // Add a CLI argument `--count`/-n` that defaults to 3, and has this help text:
    /// How many lines to get
    #[structopt(long = "count", short = "n", default_value = "3")]
    count: usize,
    // Add a positional argument that the user has to supply:
    /// The file to read
    file: String,
    // Quick and easy logging setup you get for free with quicli
    #[structopt(flatten)]
    verbosity: Verbosity,
}
fn main() -> CliResult {
    let args = Cli::from_args();

    read_file(&args.file)?
        .lines()
        .take(args.count)
        .for_each(|line| println!("{}", line));
    
    Ok(())
}