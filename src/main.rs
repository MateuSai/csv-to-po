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
    let mut pot_file_path = otput_dir.clone();
    pot_file_path.push("template.pot");
    let pot_file = match File::create(&pot_file_path) {
        Err(why) => panic!("couldn't create {}: {}", pot_file_path.display(), why),
        Ok(file) => file,
    };
    
    let mut pot_bw = BufWriter::new(&pot_file);
    pot_bw.write("msgid \"\"\nmsgstr \"\"\n".as_bytes())?;
    
    pot_bw.write("\"MIME-Version: 1.0\\n\"\n".as_bytes())?;
    pot_bw.write("\"Content-Type: text/plain; charset=UTF-8\\n\"\n".as_bytes())?;
    pot_bw.write("\"Content-Transfer-Encoding: 8bit\\n\"\n\n".as_bytes())?;
    
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path(csv_path)?;
    
    //let mut lang_files = Vec::new();
    let mut po_bws = Vec::new();
    let mut headers = rdr.headers()?.iter();
    headers.next(); // we skip the first element, the one that represents the keys
    for lang in headers {
        let mut lang_file_path = otput_dir.clone();
        lang_file_path.push(format!("{}.po", lang));
        let mut bw = match File::create(lang_file_path) {
            Err(why) => panic!("couldn't create {}: {}", pot_file_path.display(), why),
            Ok(file) => BufWriter::new(file),
        };
        
        bw.write(format!("\"Language: {}\\n\"\n", lang).as_bytes())?;
        bw.write("\"MIME-Version: 1.0\\n\"\n".as_bytes())?;
        bw.write("\"Content-Type: text/plain; charset=UTF-8\\n\"\n".as_bytes())?;
        bw.write("\"Content-Transfer-Encoding: 8bit\\n\"\n\n".as_bytes())?;
        
        po_bws.push(bw);
    }
    
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        let mut record_iter = record.iter();
        // add the id in the pot file
        let tr_id = record_iter.next();
        if let Some(id) = tr_id {
            if !id.is_empty() {
                pot_bw.write(format!("msgid \"{}\"\n", id).as_bytes())?;
                pot_bw.write("msgstr \"\"\n\n".as_bytes())?;
                
                let mut i = 0;
                for tr in record_iter {
                    let bw = po_bws.get_mut(i).unwrap();
                    bw.write(format!("msgid \"{}\"\n", id).as_bytes())?;
                    bw.write(format!("msgstr \"{}\"\n\n", tr).as_bytes())?;
                    i += 1;
                }
            }
        }
        /*for data in record.iter() {
            bw.write(format!("{}{}", data, "\n").as_bytes())?;
        }*/
    }
    
    pot_bw.flush()?;
    for mut bw in po_bws {
        bw.flush()?;
    }
    
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