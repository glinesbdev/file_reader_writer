use file_reader_writer::{*, args::Args};
use types::Result;
use std::{env, fs};

mod types;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let args = Args::from_env(args)?;
    let mut file = open_or_create_file(&args)?;

    write_to_file(&mut file, args.contents())?;

    if args.print_contents() {
        let result = fs::read_to_string(args.filepath())?;
        println!("The contents of {} is\n{result}", args.filepath());
    }

    Ok(())
}
