use std::{
    fs::File,
    io::{ErrorKind, Write},
};

pub mod args;
mod types;

use args::Args;
use types::Result;

/// Opens or creates a file from args and returns an open file handle in write and append mode.
///
/// # Examples
///
/// ```
/// use std::env;
/// use file_reader_writer::*;
/// use args::Args;
/// let args: Vec<String> = vec![
///     "target/debug/file_reader_writer".into(),
///     "./tmp/test.txt".into(),
///     "Some content".into()
/// ]; // = env::args().collect();
/// let args = Args::from_env(args)?;
/// open_or_create_file(&args)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn open_or_create_file(args: &Args) -> Result<File> {
    match File::options()
        .write(true)
        .append(args.appendable())
        .truncate(args.truncatable())
        .open(args.filepath())
    {
        Ok(f) => Ok(f),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => Ok(File::create(args.filepath())?),
            _ => Err("Could not open file".into()),
        },
    }
}

/// Writes to an open file handle
///
/// # Examples
///
/// ```
/// use std::env;
/// use file_reader_writer::*;
/// use args::Args;
/// let args: Vec<String> = vec![
///     "target/debug/file_reader_writer".into(),
///     "./tmp/test.txt".into(),
///     "Some content".into()
/// ]; // = env::args().collect();
/// let args = Args::from_env(args)?;
/// let mut file = open_or_create_file(&args)?;
/// let contents = "My awesome file text";
///
/// write_to_file(&mut file, contents)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn write_to_file(file: &mut File, contents: &str) -> Result<()> {
    let mut permissions = file.metadata()?.permissions();
    permissions.set_readonly(false);
    file.set_permissions(permissions)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}
