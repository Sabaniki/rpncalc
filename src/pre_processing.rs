use std::io::BufRead;
use clap::{ArgMatches, App, Arg};
use crate::calculator::RpnCalculator;
use anyhow::Result;

pub fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calc = RpnCalculator::new(verbose);
    for line in reader.lines() {
        let line = line?;
        match calc.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => eprintln!("{:#?}", e),
        }
    }
    Ok(())
}

pub fn create_app() -> ArgMatches {
    App::new("mini RPN calc")
        .version("1.0.0")
        .author("sabaniki")
        .about("This is mini RPN(Reverse Polish Notation) calculator program")
        .arg(
            Arg::new("formula_file")
                .about("Formulas written in RPN")
                .value_name("FILE")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .about("Sets the level of verbosity")
                .short('v')
                .long("verbose")
                .required(false),
        )
        .get_matches()
}