use ::std::io::{self, BufRead, BufReader};
use clap::{App, Arg};
use std::error::Error;
use std::fs::File;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut line_count = 1;
                for (i, res) in file.lines().enumerate() {
                    let line = res?;
                    if config.number_lines {
                        println!("{:>6}\t{}", i + 1, line);
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!("");
                        } else {
                            println!("{:>6}\t{}", line_count, line);
                            line_count += 1;
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

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("mukadenodaiou")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .default_value("-")
                .value_name("FILE")
                .help("Input files")
                .multiple(true),
        )
        .arg(
            Arg::with_name("number_lines")
                .long("number")
                .short("n")
                .help("Print line number")
                .takes_value(false)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .long("number-nonblank")
                .short("b")
                .help("Print line number without blank lines")
                .takes_value(false)
                .conflicts_with("number_lines"),
        )
        .get_matches();
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}
