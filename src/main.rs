mod checklist;
mod helpers;

use std::{fs, path::Path, process::ExitCode};

use crate::checklist::checklist::Checklist;
use crate::helpers::logger::setup_logger;
use crate::helpers::ui::draw;
use log::{debug, error, info, warn, LevelFilter};

use clap::{Parser as clapParser};

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
        0 => {
            setup_logger(LevelFilter::Error);
            error!("Verbosity Set to Error");
        }
        1 => {
            setup_logger(LevelFilter::Warn);
            warn!("Verbosity Set to Warning");
        }
        2 => {
            setup_logger(LevelFilter::Info);
            info!("Verbosity Set to Information");
        }
        _ => {
            setup_logger(LevelFilter::Debug);
            debug!("Verbosity Set to Debug");
        }
    }
}

fn arg_to_checklist_file_path(path: Option<String>) -> String {
    match path {
        Some(string) => {
            debug!("checklist_path: {}", &string);
            return string
        },
        None => panic!("Missing checklist path"),
    }
}

fn main() -> ExitCode {
    let args = Args::parse();
    verbosity(args.verbose);

    let checklist_path = arg_to_checklist_file_path(args.checklist_path);

    if args.save {
        let checklist_from_file = load_checklist(&checklist_path); //Load the Checklist from Markdown File
        let checklist_from_ui:Checklist;
        match load_saved_checklist(&checklist_path, &checklist_from_file) {
            Ok(checklist) => {
                checklist_from_ui = draw(checklist);
            },
            Err(_) => {
                checklist_from_ui = draw(checklist_from_file);
            }
        }    
        save_checklist(&checklist_from_ui,&checklist_path);
        return ExitCode::from(checklist_from_ui.get_count_unresolved());
    } else {
        let checklist_from_ui = draw( load_checklist(&checklist_path));
        return ExitCode::from(checklist_from_ui.get_count_unresolved());
    }
}

fn load_checklist(path: &String) -> Checklist {
    let file_contents = fs::read_to_string(path);
    match file_contents {
        Ok(unparsed_checklist) => {
            return Checklist::new(unparsed_checklist);
        }
        Err(err) => {
            panic!("Something went wrong reading the file:\n{:?}", err)
        }
    }
}

fn save_checklist(checklist: &Checklist,checklist_path: &String) {
    let checklist_as_toml: String = checklist.to_toml();
    let checklist_path_dir = Path::new(checklist_path).parent().unwrap();
    let checklist_save_path = checklist_path_dir.join(format!(".{}.kb.toml",&checklist.name));
    
    match fs::write(&checklist_save_path, checklist_as_toml) {
        Ok(_) => debug!("Save Checklist progress to {}", &checklist_save_path.to_string_lossy()),
        Err(err) => error!("Couldn't save progress:\n{:?}",err)
    }
}

fn load_saved_checklist(checklist_path: &String, checklist: &Checklist) -> Result<Checklist, &'static str> {
    let checklist_path_dir = Path::new(checklist_path).parent().unwrap();
    let checklist_save_path = checklist_path_dir.join(format!(".{}.kb.toml",&checklist.name));
    if checklist_save_path.exists() {
        let file_contents = fs::read_to_string(checklist_save_path);
        match file_contents {
            Ok(unparsed_checklist) => {
                return Ok(Checklist::from_toml(unparsed_checklist, checklist.name.clone()));
            }
            Err(err) => {
                error!("Failed to load save file:\n{:?}",err);
                return Err("Failed to load save file");
            }
        }
    } else {
        error!("No save file found");
        return Err("No save file found");
    }
}