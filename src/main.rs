#[macro_use]
extern crate log;

extern crate yaml_rust;
use clap::{Arg, App};
use std::fs;
use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;
use yaml2env::convert;
use yaml2env::Result;


fn main() -> Result<()> {
  env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
  let flags = App::new("yaml2env")
        .version("0.1.0")
        .author("vikin91 <vikin91@users.noreply.github.com>")
        .about("Converts first level variables from yaml file into set of exportable bash variables in a 'env' file")
        .arg(
          Arg::new("in-file")
                 .short('i')
                 .long("in")
                 .takes_value(true)
                 .required(true)
                 .about("Path of the input yaml file")
        )
        .arg(
          Arg::new("out-file")
                 .short('o')
                 .long("out")
                 .takes_value(true)
                 .default_value_os( OsStr::new("out.env"))
                 .about("Path of the output env file")
        )
        .arg(
          Arg::new("filter")
                 .short('f')
                 .long("filter")
                 .takes_value(true)
                 .multiple(true)
                 .about("List of yaml keys to export")
        )
        .get_matches();

  let in_file = Path::new(flags.value_of_os("in-file").unwrap()); // safe to unwrap because the argument is required
  debug!("Input file: {:?}", in_file.to_str().unwrap());

  let input_str = match fs::read_to_string(in_file) {
    Ok(s) => s,
    Err(e) => {
      error!("Unable to open file '{}': {}", in_file.to_str().unwrap(), e.to_string());
      std::process::exit(1);
    }
  };
  let result = convert(input_str)?;

  let out_file = flags.value_of_os("out-file").unwrap(); // safe to unwrap because default is provided
  debug!("Output file: {:?}", out_file.to_str().unwrap());

  let out_f = File::create(out_file).unwrap();
  let mut buf = BufWriter::new(out_f);
  buf.write_all(result.as_bytes())?;
  buf.flush()?;
  Ok(())
}
