#![warn(clippy::all)]
#[macro_use]
extern crate log;

extern crate yaml_rust;
use clap::{Arg, Command};
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;
use yaml2env::convert;
use yaml2env::Result;

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let flags = Command::new("yaml2env")
        .version("0.1.0")
        .author("vikin91 <vikin91@users.noreply.github.com>")
        .about(
            "Converts first level variables from a yaml file into set of exportable shell variables",
        )
        .arg(
            Arg::new("in-file")
                .short('i')
                .long("in")
                .takes_value(true)
                .required(true)
                .help("Path of the input yaml file"),
        )
        .arg(
            Arg::new("out-file")
                .short('o')
                .long("out")
                .takes_value(true)
                .default_value_os(OsStr::new("out.env"))
                .help("Path of the output env file"),
        )
        .arg(
            Arg::new("filter")
                .short('f')
                .long("filter")
                .use_value_delimiter(true)
                .help("Comma-separated list of yaml keys to export"),
        )
        .get_matches();

    let in_file = Path::new(flags.value_of_os("in-file").unwrap()); // safe to unwrap because the argument is required
    debug!("Input file: {:?}", in_file.to_str().unwrap());

    let input_str = match fs::read_to_string(in_file) {
        Ok(s) => s,
        Err(e) => {
            error!(
                "Unable to open file '{}': {}",
                in_file.to_str().unwrap(),
                e.to_string()
            );
            std::process::exit(1);
        }
    };
    let filter_param = build_filter(flags.values_of("filter"));
    let out_file = flags.value_of_os("out-file").unwrap(); // safe to unwrap because default is provided
    debug!("Output file: {:?}", out_file.to_str().unwrap());
    write_result(convert(input_str, filter_param.as_slice())?, out_file)
}

fn write_result(s: String, dest: &OsStr) -> Result<()> {
    let out_f = File::create(dest).unwrap();
    let mut buf = BufWriter::new(out_f);
    buf.write_all(s.as_bytes())?;
    buf.flush()?;
    Ok(())
}

fn build_filter(values: Option<clap::Values>) -> Vec<String> {
    match values {
        Some(l) => l
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
        None => Vec::<String>::new(),
    }
}
