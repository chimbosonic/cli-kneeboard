mod checklist;
mod helpers;

use std::{error, fs, path::Path, process::ExitCode};

use crate::checklist::Checklist;
use crate::helpers::logger::setup_logger;
use crate::helpers::ui::draw;
use log::{debug, error, info, warn, LevelFilter};

use clap::Parser as clapParser;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(clapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the checklist
    #[clap(short, long, value_parser, required(true))]
    checklist_path: String,

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

fn main() -> ExitCode {
    main_sub().unwrap_or_else(|err| {
        error!("Error: {}", err);
        ExitCode::FAILURE
    })
}

fn main_sub() -> Result<ExitCode> {
    let args = Args::parse();
    verbosity(args.verbose);

    let file_contents = fs::read_to_string(&args.checklist_path)?;
    let checklist_from_file = Checklist::from_markdown(file_contents)?;

    let checklist_from_ui: Checklist;

    if args.save {
        checklist_from_ui = match load_saved_checklist(&args.checklist_path, &checklist_from_file) {
            Ok(checklist) => draw(checklist),
            Err(_) => draw(checklist_from_file),
        };
        save_checklist(&checklist_from_ui, &args.checklist_path)?;
    } else {
        checklist_from_ui = draw(checklist_from_file);
    }

    Ok(ExitCode::from(checklist_from_ui.get_count_unresolved()))
}

fn save_checklist(checklist: &Checklist, checklist_path: &String) -> Result<()> {
    let checklist_as_toml = checklist.to_toml()?;
    let checklist_path_dir = Path::new(checklist_path)
        .parent()
        .ok_or("Failed to find parent dir path")?;
    let checklist_save_path = checklist_path_dir.join(format!(".{}.kb.toml", &checklist.name));
    fs::write(&checklist_save_path, checklist_as_toml)?;
    debug!(
        "Save Checklist progress to {}",
        &checklist_save_path.to_string_lossy()
    );
    Ok(())
}

fn load_saved_checklist(checklist_path: &String, checklist: &Checklist) -> Result<Checklist> {
    let checklist_path_dir = Path::new(checklist_path)
        .parent()
        .ok_or("Failed to find parent dir path")?;
    let checklist_save_path = checklist_path_dir.join(format!(".{}.kb.toml", &checklist.name));
    if checklist_save_path.exists() {
        let file_contents = fs::read_to_string(checklist_save_path)?;
        Checklist::from_toml(file_contents)
    } else {
        error!("No save file found");
        Err("No save file found".into())
    }
}
