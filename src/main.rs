mod libs;
use clap::Parser as clapParser;


/// Simple program to greet a person
#[derive(clapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the checklist
    #[clap(short, long, value_parser)]
    checklist_path: Option<String>,

    /// Save progress of the checklist
    #[clap(short, long, value_parser)]
    save: bool,

    /// Turn debugging information on
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

// TODO: Build log level changer
fn verbosity(level: u8) {
    match level {
        0 => println!("Verbose mode is off"),
        1 => println!("Verbose mode is kind of on"),
        2 => println!("Verbose mode is on"),
        _ => println!("Don't be crazy"),
    }
}

fn main() {
    let args = Args::parse();
    verbosity(args.verbose);

    println!("checklist_path: {:?}", args.checklist_path);
    println!("save: {:?}", args.save);
}



