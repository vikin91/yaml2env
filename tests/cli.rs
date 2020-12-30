use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*;
use log::{info, debug};
use predicates::prelude::*; // Used for writing assertions
use std::{process::Command}; // Run programs

#[test]
fn test_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("yaml2env")?;

    cmd.arg("-h");
    cmd.assert()
        .success()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::contains("USAGE:"))
        .stdout(predicate::str::contains("yaml2env [OPTIONS]"));

    Ok(())
}
#[test]
fn run_short_flag() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("yaml2env")?;

    let input_file = assert_fs::NamedTempFile::new("test.yaml").unwrap();
    let output_file = assert_fs::NamedTempFile::new("test.env").unwrap();
    input_file.touch().unwrap();
    input_file.write_str("variable: value").unwrap();
    input_file.assert(predicate::str::contains("variable: value"));
    info!("{}", output_file.path().to_str().unwrap());

    cmd
        .arg("-i")
        .arg(input_file.path().to_str().unwrap())
        .arg("-o")
        .arg(output_file.path().to_str().unwrap());
    cmd.assert()
        .success();

    output_file.assert(predicate::str::contains("bash"));
    output_file.assert(predicate::str::contains("variable"));
    output_file.assert(predicate::str::contains("value"));

    input_file.close().unwrap();
    output_file.close().unwrap();
    Ok(())
}

#[test]
fn run_long_flag() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("yaml2env")?;

    let input_file = assert_fs::NamedTempFile::new("test.yaml").unwrap();
    let output_file = assert_fs::NamedTempFile::new("test.env").unwrap();
    input_file.touch().unwrap();
    input_file.write_str("variable: value").unwrap();
    debug!("{}", output_file.path().to_str().unwrap());

    cmd
        .arg("--in")
        .arg(input_file.path().to_str().unwrap())
        .arg("--out")
        .arg(output_file.path().to_str().unwrap());
    cmd.assert()
        .success();

    output_file.assert(predicate::str::contains("bash"));
    output_file.assert(predicate::str::contains("variable"));
    output_file.assert(predicate::str::contains("value"));

    input_file.close().unwrap();
    output_file.close().unwrap();
    Ok(())
}

#[test]
fn empty_input() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("yaml2env")?;

    let input_file = assert_fs::NamedTempFile::new("test.yaml").unwrap();
    let output_file = assert_fs::NamedTempFile::new("test.env").unwrap();
    input_file.touch().unwrap();
    debug!("{}", output_file.path().to_str().unwrap());

    cmd
        .arg("--in")
        .arg(input_file.path().to_str().unwrap())
        .arg("--out")
        .arg(output_file.path().to_str().unwrap());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("input file contains no yaml documents"));

    input_file.assert(predicate::str::is_empty());
    output_file.assert(predicate::path::missing());

    input_file.close().unwrap();
    output_file.close().unwrap();
    Ok(())
}

#[test]
fn empty_first_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("yaml2env")?;

    let input_file = assert_fs::NamedTempFile::new("test.yaml").unwrap();
    let output_file = assert_fs::NamedTempFile::new("test.env").unwrap();
    input_file.touch().unwrap();
    input_file.write_str("---\n\
    ---\n\
    second_file: here
    ---\n\
    ").unwrap();
    debug!("{}", output_file.path().to_str().unwrap());

    cmd
        .arg("--in")
        .arg(input_file.path().to_str().unwrap())
        .arg("--out")
        .arg(output_file.path().to_str().unwrap());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("first yaml document is empty"));

    output_file.assert(predicate::path::missing());

    input_file.close().unwrap();
    output_file.close().unwrap();
    Ok(())
}

#[test]
fn two_yamls() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("yaml2env")?;

    let input_file = assert_fs::NamedTempFile::new("test.yaml").unwrap();
    let output_file = assert_fs::NamedTempFile::new("test.env").unwrap();
    input_file.touch().unwrap();
    input_file.write_str("---\n\
    first_file: here
    ---\n\
    second_file: here
    ---\n\
    ").unwrap();
    debug!("{}", output_file.path().to_str().unwrap());

    cmd
        .arg("--in")
        .arg(input_file.path().to_str().unwrap())
        .arg("--out")
        .arg(output_file.path().to_str().unwrap());
    cmd.assert()
        .success();
        // .stderr(predicate::str::contains("first yaml document is empty"));

    output_file.assert(predicate::path::exists());
    output_file.assert(predicate::str::contains("first_file="));
    output_file.assert(predicate::str::contains("here"));

    input_file.close().unwrap();
    output_file.close().unwrap();
    Ok(())
}

#[test]
fn filter() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("yaml2env")?;

    let input_file = assert_fs::NamedTempFile::new("test.yaml").unwrap();
    let output_file = assert_fs::NamedTempFile::new("test.env").unwrap();
    input_file.touch().unwrap();
    input_file.write_str("---\n\
    var1: val1\n\
    var2: val2\n\
    var3: val3\n\
    ").unwrap();
    debug!("{}", output_file.path().to_str().unwrap());

    cmd
        .arg("--in")
        .arg(input_file.path().to_str().unwrap())
        .arg("--out")
        .arg(output_file.path().to_str().unwrap())
        .arg("--filter")
        .arg("var1,var3");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains(" Written key 'var1'"));

    output_file.assert(predicate::path::exists());
    output_file.assert(predicate::str::contains("var1="));
    output_file.assert(predicate::str::is_match(r#"var\d="#).unwrap().count(2));

    input_file.close().unwrap();
    output_file.close().unwrap();
    Ok(())
}
#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("yaml2env")?;

    cmd
        .arg("-i")
        .arg("test/file/doesnt/exist")
        .arg("-o")
        .arg("out.env");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Unable to open file"))
        .stderr(predicate::str::contains("No such file or directory"));
    Ok(())
}
