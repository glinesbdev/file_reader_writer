use std::path::Path;

use crate::types::Result;

/// Struct to store command line arguments
pub struct Args {
    filepath: String,
    contents: String,
    append: bool,
    truncate: bool,
    print_contents: bool,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            filepath: "".into(),
            contents: "".into(),
            append: false,
            truncate: false,
            print_contents: true,
        }
    }
}

impl Args {
    /// Filepath from args
    pub fn filepath(&self) -> &String {
        &self.filepath
    }

    /// Contents from args
    pub fn contents(&self) -> &String {
        &self.contents
    }

    /// Flag given to the program i.e. `--append`.
    /// If present, it appends `contents` to the file in the `filepath`.
    /// If not present, it overwrites the contents of the file in `filepath`.
    pub fn appendable(&self) -> bool {
        self.append
    }

    /// Flag given to the program i.e. `--truncate`.
    /// If present, truncates the file in `filepath` before writing `contents` to it.
    pub fn truncatable(&self) -> bool {
        self.truncate
    }

    /// Flag given to the program i.e. `--no-print`.
    /// If present, it doesn't print the output of the written file to stdout.
    pub fn print_contents(&self) -> bool {
        self.print_contents
    }

    /// Collects command line arguments. Takes full ownership of the `args` argument;
    ///
    /// # Examples
    ///
    /// ```
    /// use std::env;
    /// use file_reader_writer::args::Args;
    /// let args: Vec<String> = vec![
    ///     "target/debug/file_reader_writer".into(),
    ///     "./tmp/test.txt".into(),
    ///     "Some content".into()
    /// ]; // = env::args().collect();
    /// let args = Args::from_env(args)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_env(args: Vec<String>) -> Result<Self> {
        let (flags, args): (Vec<String>, Vec<String>) =
            args.into_iter().skip(1).partition(|s| s.starts_with("-"));

        let mut result = Self::default();

        result.parse_flags(&flags[..]);
        result.filepath = args.get(0).ok_or_else(|| "Filepath required")?.to_string();

        if !Path::is_file(Path::new(&result.filepath)) {
            return Err("Filepath arg must be a file".into());
        }

        if let Some(contents) = args.get(1) {
            result.contents = contents.into();
        }

        Ok(result)
    }

    fn parse_flags(&mut self, flags: &[String]) {
        for arg in &flags[..] {
            match arg.as_str() {
                "--append" | "-a" => self.append = true,
                "--truncate" | "-t" => self.truncate = true,
                "--no-print" | "-np" => self.print_contents = false,
                "--help" | "-h" => Self::print_help(),
                _ => (),
            }
        }
    }

    fn print_help() {
        let version = env!("CARGO_PKG_VERSION");

        println!(
            r#"
File Reader Writer - {version}

Usage: file_reader_writer [filepath] [contents] [OPTIONS]
Example: file_reader_writer ./tmp/my_file.txt "Here is some content." -t

Options:
    -a, --append        Appends new [contents] to the [filepath] file.
    -t, --truncate      Truncates the [filepath] file before writing [contents] to it.
                        Ignores --append option when used.
    -np, --no-print     Doesn't print the output of the file after writing.
    -h, --help          Shows this help screen.
            "#
        );

        std::process::exit(0);
    }
}
