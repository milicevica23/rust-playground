use std::path::Path;
use color_eyre::{Result};
use reqwest::blocking::Client;
use polars::prelude::*;
use std::io::Cursor;

fn download_csv(url: &str, path_to_save: &Path) -> Result<()>{
    let data: Vec<u8> = Client::new()
        .get(url)
        .send()?
        .text()?
        .bytes()
        .collect();
    let mut df = CsvReader::new(Cursor::new(data))
        .has_header(true)
        .finish()?;
     
    let mut file = std::fs::File::create(path_to_save).unwrap();
    CsvWriter::new(&mut file).finish(& mut df).unwrap();
    println!("{:?}", df.shape());
    println!("{:?}",df);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        download_csv("https://j.mp/iriscsv", Path::new("./iris.csv"));
    }
}
