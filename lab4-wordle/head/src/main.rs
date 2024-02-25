use clap::Parser;
use std::io::{self, BufReader, BufRead, Write};
use std::fs::File;
use std::path::PathBuf;


#[derive(Parser, Debug)]
#[command(author, version = "1.2.3", about = "Display the first few lines of a file", long_about = None)]
struct Config {
    /// Print LINES lines of each of the specified files, 10 bytes by default
    #[arg(short = 'n', long, default_value = "10")]
    lines: Option<usize>,
    
    /// Print BYTES bytes of each of the specified files, 10 bytes by default
    #[arg(short = 'c', long, conflicts_with = "lines")]
    bytes: Option<usize>,

    /// Input files or - for stdin
    #[arg(default_value = "-")]
    files: Vec<PathBuf>,
}

/// Print `count` lines of `input` to `stdout`.
fn print_lines(input: &mut dyn BufRead, count: usize) -> io::Result<()> { 
    for _i in 0..count {
        let mut line = String::new();
        input.read_line(&mut line)?;
        print!("{line}");
    }
    Ok(())
 }

fn print_bytes(input: &mut dyn BufRead, count: usize) -> io::Result<()> { 
    let mut bytes = Vec::new();
    bytes.resize(count, 0); // Fill it with count zero bytes.
    let length = input.read(&mut bytes)?;
    io::stdout().write_all(&bytes[..length])?;
    Ok(())
 }

fn main() {
    let config = Config::parse();
    let mut failure = false;
    let mut header = String::new();

    for file in &config.files {
        if config.files.len() > 1 {
            if file.to_str() == Some("-") {          // prints the header for standard input
                header = String::from("==> standard input <==");
            } else {
                header = format!("==> {} <==", file.to_str().unwrap()); // prints the header for each file
            }  
        }

        if file.to_str() == Some("-") {
            let stdin = io::stdin();
            let mut reader = Box::new(BufReader::new(stdin.lock()));
            if config.bytes.is_some() {
                if let Err(err) = {
                    println!("{header}");
                    print_bytes(&mut reader, config.bytes.unwrap())} {
                    eprintln!("Error: {}", err);
                    failure = true;
                } 
            } else {
                if let Err(err) = {
                    println!("{header}");
                    print_lines(&mut reader, config.lines.unwrap())} {
                    eprintln!("Error: {}", err);
                    failure = true;
                }
            }

        } else {
            match File::open(file) {
                Ok(open_file) => {
                    let mut reader = Box::new(BufReader::new(open_file));
                    println!("{header}");
                    if config.bytes.is_some() {
                        let _ = print_bytes(&mut reader, config.bytes.unwrap());
                    } else {
                        let _ = print_lines(&mut reader, config.lines.unwrap());
                    }
                } 
                Err(err) => {
                    eprintln!("{}: {err}", file.display());
                    failure = true;
                }
                
            };
        }
        println!("\n");
    }

    if failure {
        std::process::exit(1);
    }
}