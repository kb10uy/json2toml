use std::{
    env::args,
    fmt::Display,
    fs::File,
    io::{prelude::*, stdin, stdout, Error as IoError},
    process::exit,
};

/// Adds `unwrap_or_exit` method.
pub trait UnwrapOrExit {
    type Output;
    fn unwrap_or_exit(self, info: &str) -> Self::Output;
}

impl<T, E: Display> UnwrapOrExit for Result<T, E> {
    type Output = T;
    fn unwrap_or_exit(self, info: &str) -> T {
        match self {
            Ok(value) => value,
            Err(error) => {
                eprintln!("{}: {}", info, error);
                exit(1);
            }
        }
    }
}

/// Obtains `dyn Write` and `dyn Read` based on commandline argument.
pub fn inout_from_args() -> Result<(Box<dyn Write>, Box<dyn Read>), IoError> {
    let mut arguments = args();
    arguments.next();

    let input: Box<dyn Read> = if let Some(input_filename) = arguments.next() {
        Box::new(File::open(input_filename)?)
    } else {
        Box::new(stdin())
    };
    let output: Box<dyn Write> = if let Some(output_filename) = arguments.next() {
        Box::new(File::create(output_filename)?)
    } else {
        Box::new(stdout())
    };

    Ok((output, input))
}
