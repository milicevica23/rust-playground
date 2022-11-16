
use assert_fs::prelude::FileWriteStr;
use clap::Parser;
use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    pattern: String,

    #[arg(short, long)]
    file_path: std::path::PathBuf,
}

pub fn find_patern_in_file( pattern: &String,
                            file_path: impl AsRef<std::path::Path>,
                            mut writer: impl std::io::Write) {
    let f1 = File::open(file_path).expect("file not found");
    let mut reader = BufReader::new(f1);

    loop {
        let mut line = String::new();
        let result = reader.read_line(&mut line);

        match result {
            Ok(0) => break,
            Ok(_) => {
                if line.contains(pattern) {
                    writeln!(writer, "{}", line).expect("writer is broken!");
                }
            },
            Err(_) => panic!("this is a terrible mistake!"),
        }
    }


}

fn main() {
    let args = Cli::parse();
    find_patern_in_file(&args.pattern, &args.file_path ,&mut std::io::stdout())
}

#[test]
fn find_a_match() -> Result<(), Box<dyn std::error::Error>> {
    let mut result = Vec::new();
    
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    //let mut file = tempfile()?;

    file.write_str("A test\nActual content\nMore content\nAnother test")?;
    
    find_patern_in_file(&String::from("test"), 
                        file.path(),
                        &mut result);
    assert_eq!(result, b"A test\n\nAnother test\n");
    Ok(())
}