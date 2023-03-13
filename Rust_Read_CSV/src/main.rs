use csv;
use std::error::Error;

fn read_from_file(path: &str) -> Result<(), Box<dyn Err>> {
    let mut reader = csv::Reaader::from_path(path)?;

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    ok(())
}

fn main() {
    if let Err(e) = read_from_file("./.csv") {
        eprintln!("{}", e);
    }
}
