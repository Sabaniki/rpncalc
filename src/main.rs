use clap::Clap;

#[derive(Clap, Debug)]
#[clap(
    name = "mini RPN calc",
    version = "1.0.0",
    author = "sabaniki",
    about = "This is mini RPN(Reverse Polish Notation) program"
)]

struct Opts {
    // Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    // Formulas written in PRN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();
    match opts.formula_file {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }
    println!("is verbosity specified?: {}", opts.verbose);
}
