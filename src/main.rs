use std::error::Error;
use std::fs::File;
use std::io::{Write, BufWriter};
use std::process;
use clap::Parser;

#[derive(Parser)]
struct Args {
    csv_file_path: std::path::PathBuf,
    
    output_directory: Option<std::path::PathBuf>,
}

fn generate_po(csv_path: std::path::PathBuf, otput_dir: std::path::PathBuf) -> Result<(), Box<dyn Error>> {
    let mut pot_file_path = otput_dir;
    pot_file_path.push("template.pot");
    println!("{:?}", pot_file_path);
    let pot_file = match File::create(&pot_file_path) {
        Err(why) => panic!("couldn't create {}: {}", pot_file_path.display(), why),
        Ok(file) => file,
    };
    
    let mut bw = BufWriter::new(&pot_file);
    bw.write("msgid \"\"\nmsgstr \"\"\n".as_bytes())?;
    // Build the CSV reader and iterate over each record.
    let rdr = csv::Reader::from_path(csv_path);
    for result in rdr?.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        for data in record.iter() {
            bw.write(format!("{}{}", data, "\n").as_bytes())?;
        }
    }
    
    bw.flush()?;
    
    Ok(())
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args.csv_file_path);
    
    let output_dir = match args.output_directory {
        Some(dir) => dir,
        None => std::path::Path::new(".").to_path_buf(),
    };

    println!("{:?}", output_dir);
    
    if let Err(err) = generate_po(args.csv_file_path, output_dir) {
        println!("error converting to po: {}", err);
        process::exit(1);
    }
}