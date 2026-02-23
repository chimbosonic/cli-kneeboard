mod checklist;
mod helpers;

use crate::checklist::Checklist;
use crate::helpers::logger::setup_logger;
use crate::helpers::ui::draw;
use log::{LevelFilter, debug, error, info, warn};
use std::{error, fs, path::Path, process::ExitCode};
use xxhash_rust::xxh3::xxh3_64;

use clap::Parser as clapParser;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(clapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the checklist
    #[clap(short, long, value_parser, required(true))]
    checklist_path: String,

    /// Save and load progress of the checklist
    #[clap(short, long, value_parser)]
    save: bool,

    /// Turn debugging information on
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Headless mode
    #[clap(long, value_parser)]
    headless: bool,
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
    let save_and_load = args.save;
    let headless_mode = args.headless;

    let file_contents = fs::read_to_string(&args.checklist_path)?;
    let mut checklist = Checklist::from_markdown(file_contents)?;

    if save_and_load {
        match load_saved_checklist(&args.checklist_path, &checklist) {
            Ok(checklist_loaded) => checklist.merge_checklist(&checklist_loaded),
            Err(error) => log::error!("Failed to load saved checklist: {error}"),
        };
    }

    if !headless_mode {
        checklist = draw(checklist);
    }

    if save_and_load {
        match save_checklist(&checklist, &args.checklist_path) {
            Ok(_) => log::info!("Saved Checklist progress to {}", &args.checklist_path),
            Err(error) => log::error!("Failed to save Checklist progress: {error}"),
        };
    }

    Ok(ExitCode::from(
        std::cmp::min(checklist.get_count_unresolved(), 255) as u8,
    ))
}

fn get_save_file_name(checklist_name: &String) -> String {
    let checklist_name_hash = xxh3_64(checklist_name.as_bytes());
    format!("{checklist_name_hash:x}")
}

fn save_checklist(checklist: &Checklist, checklist_path: &String) -> Result<()> {
    let checklist_as_toml = checklist.to_toml()?;
    let checklist_path_dir = Path::new(checklist_path)
        .parent()
        .ok_or("Failed to find parent dir path")?;
    let checklist_save_path =
        checklist_path_dir.join(format!(".{}.kb.toml", get_save_file_name(&checklist.name)));
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
    let checklist_save_path =
        checklist_path_dir.join(format!(".{}.kb.toml", get_save_file_name(&checklist.name)));
    if checklist_save_path.exists() {
        let file_contents = fs::read_to_string(checklist_save_path)?;
        Checklist::from_toml(file_contents)
    } else {
        Err("No save file found".into())
    }
}
