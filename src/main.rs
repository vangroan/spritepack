#[macro_use]
extern crate clap;
extern crate colored;
#[macro_use]
extern crate error_chain;
extern crate image;
extern crate log;
extern crate regex;
extern crate simple_logger;
extern crate walkdir;

use clap::{App, Arg};
use image::DynamicImage;
use log::{info, trace};
use regex::Regex;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub mod errors;
mod pack;

use pack::*;

fn validate_integer(s: String) -> Result<(), String> {
    let pattern = Regex::new(r"\d+").unwrap();

    if pattern.is_match(&s) {
        Ok(())
    } else {
        Err(format!("{} is not a valid integer", s))
    }
}

fn main() -> errors::Result<()> {
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
        .arg(
            Arg::with_name("name")
                .default_value("out.png")
                .short("n")
                .long("name")
                .takes_value(true)
                .help("Filename of output image")
        )
        .arg(
            Arg::with_name("width")
                .default_value("1024")
                .short("w")
                .long("width")
                .takes_value(true)
                .validator(validate_integer)
                .help("Width of output image")
        )
        .arg(
            Arg::with_name("height")
                .default_value("1024")
                .short("h")
                .long("height")
                .takes_value(true)
                .validator(validate_integer)
                .help("Height of output image")
        )
        .get_matches();

    info!("Starting");

    let file_extensions: HashSet<&&'static str> = ["jpg", "jpeg", "png"].into_iter().collect();

    let input_folder = Path::new(
        matches
            .value_of("folder")
            .expect("Input folder not provided"),
    );
    let folder_path = fs::canonicalize(input_folder).expect("Failed to canonicalize input path");

    let _recurse = matches.is_present("recursive");
    let output_filename = matches.value_of("name").unwrap();
    let width = matches.value_of("width").unwrap().parse::<u32>().unwrap();
    let height = matches.value_of("height").unwrap().parse::<u32>().unwrap();

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

    let images: Vec<DynamicImage> = paths
        .iter()
        .map(|path| {
            trace!("Loading {}", path.display());
            image::open(path).expect("Failed loading image")
        })
        .collect();

    info!("Packing {} images", images.len());
    let mut packer = Packer::new((width, height));
    let output = packer.pack(images)?;

    info!("Outputting {} ({} x {})", output_filename, width, height);
    output.save(output_filename)?;

    info!("Done");

    Ok(())
}
