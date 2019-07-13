#[macro_use]
extern crate clap;
extern crate colored;
extern crate image;
extern crate log;
extern crate simple_logger;
extern crate walkdir;

use clap::{App, Arg};
use log::{info, trace, warn};
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod pack;

use pack::*;

fn main() {
    // Older Windows CMD does not support coloured output
    #[cfg(windows)]
    {
        if !ansi_term::enable_ansi_support().is_ok() {
            colored::control::set_override(false);
        }
    }

    simple_logger::init().unwrap();

    let matches = App::new("Sprite Packer")
        .version(crate_version!())
        .author("Willem Victor <wimpievictor@gmail.com>")
        .about("Command line tool for combining sprites into texture files")
        .arg(
            Arg::with_name("folder")
                .required(true)
                .value_name("FOLDER")
                .help("Relative path to folder containing images"),
        )
        .arg(
            Arg::with_name("recursive")
                .required(false)
                .short("r")
                .takes_value(false)
                .help("Walks down into folders when true, otherwise only uses files in current directory")
        )
        .get_matches();

    info!("Starting");

    let _packer = Packer::new((512, 512));
    let file_extensions: HashSet<&&'static str> = ["jpg", "jpeg", "png"].into_iter().collect();

    let input_folder = Path::new(
        matches
            .value_of("folder")
            .expect("Input folder not provided"),
    );
    let folder_path = fs::canonicalize(input_folder).expect("Failed to canonicalize input path");

    let recurse = matches.is_present("recursive");

    // Walk Directory
    if recurse {
        trace!("Walking {}", folder_path.display());

        let paths: Vec<PathBuf> = WalkDir::new(folder_path)
            .into_iter()
            .map(|result| result.unwrap())
            .filter(|entry| entry.file_type().is_file())
            .map(|entry| entry.into_path())
            .filter(|path| {
                path.extension()
                    .and_then(OsStr::to_str)
                    .map(|ext| file_extensions.contains(&ext))
                    .unwrap_or(false)
            })
            .collect();

        for path in paths {
            trace!("Found {}", path.display());
        }
    } else {
        unimplemented!()
    }

    // TODO: Subtract CWD from File Path
    // TODO: Load Images into memory
    // TODO: Pack Images
}
