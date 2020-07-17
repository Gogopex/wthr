use std::env::args;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() {
    let args = Cli::from_args();
    let path = "test.txt";
    let content = std::fs::read_to_string(path)
    .map_err(|err| CustomError(format!("error reading file: {} {}", path, err)));
    
    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
}
