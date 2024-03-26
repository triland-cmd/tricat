use std::error::Error;

use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
/// Rust cat
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-", num_args(1..)
)]
    files: Vec<String>,
    /// Number the output lines, starting at 1
    #[arg(
        short = 'n',
        long,
        default_value_t = false,
        conflicts_with("number_nonblank")
    )]
    number: bool,
    /// Squeeze multiple adjacent empty lines
    #[arg(short = 'b', long, default_value_t = false)]
    number_nonblank: bool,
}

pub fn run() -> MyResult<()> {
    let args = Cli::parse();
    dbg!(&args);
    for filename in &args.files {
        match open(filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line) in file.lines().enumerate() {
                    let line = line?;
                    if args.number {
                        println!("{:>6}\t{}", line_num + 1, line);
                    } else if args.number_nonblank {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:>6}\t{}", last_num, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}
