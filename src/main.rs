mod pre_processing;
mod calculator;
mod tests;

use pre_processing::{run, create_app};
use std::fs::File;
use std::io::{BufReader, stdin};
use std::process;
use anyhow::Result;

fn main() -> Result<()> {
    let matches = create_app();

    if let Some(path) = matches.value_of("formula_file") {
        let file;
        match File::open(path) {
            Ok(f) => file = f,
            Err(_) => {
                eprintln!("The specified file could not be found");
                process::exit(1);
            }
        }

        let reader = BufReader::new(file);
        run(reader, matches.is_present("verbose"))
    } else {
        // ファイルを指定しなかった場合
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, matches.is_present("verbose"));
        println!("No file is specified");
    }
}



