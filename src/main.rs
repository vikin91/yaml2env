#[macro_use]
extern crate simple_error;

extern crate yaml_rust;
use yaml_rust::{YamlLoader};
use clap::{Arg, App};
use std::fs;
use std::io::BufWriter;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;

type BoxResult<T> = Result<T,Box<dyn Error>>;

fn main() -> BoxResult<()> {
  let flags = App::new("yaml2env")
        .version("0.1.0")
        .author("vikin91 <vikin91@users.noreply.github.com>")
        .about("Converts first level variables from yaml file into set of exportable bash variables in a 'env' file")
        .arg(Arg::with_name("in-file")
                 .short("i")
                 .long("in")
                 .takes_value(true)
                 .help("Path of the input yaml file"))
        .arg(Arg::with_name("out-file")
                 .short("o")
                 .long("out")
                 .takes_value(true)
                 .help("Path of the output env file"))
        .arg(Arg::with_name("filter")
                 .short("f")
                 .long("filter")
                 .takes_value(true)
                 .default_value("")
                 .empty_values(true)
                 .multiple(true)
                 .help("List of yaml keys to export"))
        .get_matches();

  let in_file_opt = flags.value_of("in-file");
  let out_file_opt = flags.value_of("out-file");
  let _filter_opt = flags.value_of("filter");

  let in_file = match in_file_opt {
    Some(inner) => inner,
    None => bail!("Input file missing")
  };

  let out_file = match out_file_opt {
    Some(inner) => inner,
    None => "out.env"
  };

  println!("Would write to '{}'", out_file);

  let input_str = match fs::read_to_string(in_file){
    Ok(content) => content,
    Err(e) => bail!("Error reading from file '{}'. Error: {}", in_file, e)
  };
  let out = match YamlLoader::load_from_str(input_str.as_str()){
    Ok(yaml) => yaml,
    Err(e) => bail!("Unable to parse file '{}' as YAML. Error: {}", in_file, e)
  };

  let first_doc = out.into_iter().next().unwrap();
  let iter = first_doc.into_hash().unwrap().into_iter();

  let out_f = match File::create(out_file){
    Ok(file) => file,
    Err(e) => bail!("Unable to open file for writing '{}'. Error: {}", out_file, e)
  };

  let mut buf = BufWriter::new(out_f);
  buf.write_all(b"#!/bin/bash\n\n")?;

  for (k, v) in iter {
    let k_opt = k.into_string();
    let v_opt = v.into_string();

    if let Some(key) = &k_opt {
      if let Some(value) = &v_opt {
        buf.write_fmt(format_args!("{name}=$(cat << '_EOF'\n{value:?}\n_EOF\n)\n\n",name=key, value=value))?;
      }
    }

  }
  buf.flush()?;
  Ok(())
}
