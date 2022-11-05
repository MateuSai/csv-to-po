use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::process;

#[derive(Parser)]
struct Args {
    #[arg(value_hint = clap::ValueHint::FilePath)]
    csv_file_path: std::path::PathBuf,

    #[arg(value_hint = clap::ValueHint::DirPath)]
    output_directory: Option<std::path::PathBuf>,

    #[arg(short, long, default_value_t = String::new())]
    project_name: String,
}

fn generate_po(
    csv_path: std::path::PathBuf,
    otput_dir: std::path::PathBuf,
    project_name: String,
) -> Result<(), Box<dyn Error>> {
    let mut pot_file_path = otput_dir.clone();
    pot_file_path.push("template.pot");
    let pot_file = match File::create(&pot_file_path) {
        Err(why) => panic!("couldn't create {}: {}", pot_file_path.display(), why),
        Ok(file) => file,
    };

    let mut pot_bw = BufWriter::new(&pot_file);
    pot_bw.write("msgid \"\"\nmsgstr \"\"\n".as_bytes())?;
    pot_bw.write(format!("\"Project-Id-Version: {}\\n\"\n", project_name).as_bytes())?;
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

        bw.write("msgid \"\"\nmsgstr \"\"\n".as_bytes())?;
        bw.write(format!("\"Project-Id-Version: {}\\n\"\n", project_name).as_bytes())?;
        bw.write("\"Language-Team: \\n\"\n".as_bytes())?;
        bw.write(format!("\"Language: {}\\n\"\n", lang).as_bytes())?;
        bw.write("\"Last-Translator: \\n\"\n".as_bytes())?;
        bw.write("\"PO-Revision-Date: \\n\"\n".as_bytes())?;
        bw.write("\"MIME-Version: 1.0\\n\"\n".as_bytes())?;
        bw.write("\"Content-Type: text/plain; charset=UTF-8\\n\"\n".as_bytes())?;
        bw.write("\"Content-Transfer-Encoding: 8bit\\n\"\n\n".as_bytes())?;

        po_bws.push(bw);
    }

    // Iterate over each row
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        let mut record_iter = record.iter();
        // add the id in the pot file
        let tr_id = record_iter.next();
        if let Some(id) = tr_id {
            if !id.is_empty() {
                // Add translation id in the pot file
                pot_bw.write(format!("msgid \"{}\"\n", id).as_bytes())?;
                pot_bw.write("msgstr \"\"\n\n".as_bytes())?;

                // Write the translation in each po file
                let mut i = 0;
                for tr in record_iter {
                    let tr = tr.replace("\"", "\\\"");
                    let bw = po_bws.get_mut(i).unwrap();
                    bw.write(format!("msgid \"{}\"\n", id).as_bytes())?;
                    bw.write(format!("msgstr \"{}\"\n\n", tr).as_bytes())?;
                    i += 1;
                }
            }
        }
    }

    pot_bw.flush()?;
    for mut bw in po_bws {
        bw.flush()?;
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    let output_dir = match args.output_directory {
        Some(dir) => {
            if dir.is_file() {
                eprintln!("The output directory specified is not a directory. The current directory will be used");
                std::path::Path::new(".").to_path_buf()
            } else {
                dir
            }
        }
        None => std::path::Path::new(".").to_path_buf(),
    };

    if let Err(err) = generate_po(args.csv_file_path, output_dir, args.project_name) {
        println!("error converting to po: {}", err);
        process::exit(1);
    }
}
