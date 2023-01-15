## File Reader Writer

This is a project that I used to start learning more about Rust.

The main binary is a CLI application that will write some text to a file.

## Usage

```shell
File Reader Writer - 0.1.0

Usage: file_reader_writer [filepath] [contents] [OPTIONS]
Example: file_reader_writer ./tmp/my_file.txt "Here is some content." -t

Options:
    -a, --append        Appends new [contents] to the [filepath] file.
    -t, --truncate      Truncates the [filepath] file before writing [contents] to it.
                        Ignores --append option when used.
    -np, --no-print     No printing to stdout.
    -h, --help          Shows this help screen.
```
