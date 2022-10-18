mod checklist;
mod helpers;

use crate::checklist::checklist::Checklist;
use crate::helpers::logger::setup_logger;
use log::LevelFilter;

use clap::Parser as clapParser;

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

fn verbosity(level: u8) {
    match level {
        0 => setup_logger(LevelFilter::Error),
        1 => setup_logger(LevelFilter::Warn),
        2 => setup_logger(LevelFilter::Info),
        _ => setup_logger(LevelFilter::Debug),
    }
}

fn main() {
    let args = Args::parse();
    verbosity(args.verbose);

    println!("checklist_path: {:?}", args.checklist_path);
    println!("save: {:?}", args.save);
    
    let markdown_input = "# Example Heading\nExample paragraph with **lorem** _ipsum_ text.\n<!-- checklist - name -->\n - [x] test checklist item";
    println!("Checklist as markdown:\n{}",markdown_input);
    let checklist_from_markdown = Checklist::new(String::from(markdown_input));
    println!("checklist read from markdown: {:?}",checklist_from_markdown);
    let toml = checklist_from_markdown.to_toml();
    println!("checklist as toml:\n{}",toml);
    let checklist_from_toml = Checklist::from_toml(toml,"name".to_string());
    println!("checklist read from toml: {:?}",checklist_from_toml)
}