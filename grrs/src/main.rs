
use clap::Parser;
use std::io::{BufReader, BufRead};
use std::fs::File;
mod test;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    pattern: String,

    #[arg(short, long)]
    file_path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    /*let content = std::fs::read_to_string(&args.file_path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }*/

    let f1 = File::open(&args.file_path);
    let mut reader = BufReader::new(f1.expect("error"));

    loop {
        let mut line = String::new();
        let result = reader.read_line(&mut line);

        match result {
            Ok(0) => break,
            Ok(_) => {
                if line.contains(&args.pattern) {
                    println!("{}", line);
                }
            },
            Err(_) => panic!("this is a terrible mistake!"),
        }
    }
}
