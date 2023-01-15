use assert_cmd::prelude::*;
use assert_fs::prelude::FileWriteStr;
use predicates::prelude::*;
use std::process::Command;

const BIN_NAME: &str = env!("CARGO_PKG_NAME");

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn creates_file_no_content() -> TestResult<()> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;

    cmd.arg("./tmp/test.txt");
    cmd.assert().success().stdout(predicate::str::contains(
        "The contents of ./tmp/test.txt is\n",
    ));

    Ok(())
}

#[test]
fn creates_file_with_content() -> TestResult<()> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;

    cmd.arg("./tmp/test.txt").arg("Some contents");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Some contents"));

    Ok(())
}

#[test]
fn writes_to_open_file() -> TestResult<()> {
    let file = assert_fs::NamedTempFile::new("existing_file.txt")?;
    file.write_str("Some existing data")?;

    let mut cmd = Command::cargo_bin(BIN_NAME)?;

    cmd.arg(file.path()).arg("Some new contents");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Some new contents"));

    Ok(())
}

#[test]
fn error_missing_filename() -> TestResult<()> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Filepath required"));

    Ok(())
}

#[test]
fn error_filepath_must_be_file() -> TestResult<()> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;

    cmd.arg("./tmp").arg("Some contents.");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Filepath arg must be a file"));

    Ok(())
}

#[test]
fn using_append_flags() -> TestResult<()> {
    let file = assert_fs::NamedTempFile::new("existing_file.txt")?;
    file.write_str("File with content.")?;

    let mut cmd = Command::cargo_bin(BIN_NAME)?;

    cmd.arg(file.path())
        .arg(" Some other content.")
        .arg("--append");

    cmd.assert().success().stdout(predicate::str::contains(
        "File with content. Some other content.",
    ));

    let mut cmd = Command::cargo_bin(BIN_NAME)?;

    cmd.arg(file.path()).arg(" Even more content!").arg("-a");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Some other content."))
        .stdout(predicate::str::contains("Even more content!"));

    Ok(())
}

#[test]
fn using_truncate_flags() -> TestResult<()> {
    let file = assert_fs::NamedTempFile::new("existing_file.txt")?;
    file.write_str("File with content.")?;

    let mut cmd = Command::cargo_bin(BIN_NAME)?;

    cmd.arg(file.path())
        .arg("Some new content.")
        .arg("--truncate");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("File with content.").not())
        .stdout(predicate::str::contains("Some new content"));

    let mut cmd = Command::cargo_bin(BIN_NAME)?;

    cmd.arg(file.path()).arg("Replaced content.").arg("-t");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Some new content").not())
        .stdout(predicate::str::contains("Replaced content."));

    Ok(())
}

#[test]
fn prints_help_menu() -> TestResult<()> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    let version = env!("CARGO_PKG_VERSION");

    cmd.arg("-h");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "File Reader Writer - {version}"
        )))
        .stdout(predicate::str::contains(
            "Usage: file_reader_writer [filepath] [contents] [OPTIONS]",
        ));

    let mut cmd = Command::cargo_bin(BIN_NAME)?;

    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "File Reader Writer - {version}"
        )))
        .stdout(predicate::str::contains(
            "Usage: file_reader_writer [filepath] [contents] [OPTIONS]",
        ));

    Ok(())
}

#[test]
fn using_no_print_flags() -> TestResult<()> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("./tmp/new_file.txt")
        .arg("Some content")
        .arg("--no-print");
    cmd.assert().success().stdout(predicate::str::is_empty());

    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("./tmp/new_file.txt").arg("Some content").arg("-np");
    cmd.assert().success().stdout(predicate::str::is_empty());

    Ok(())
}
