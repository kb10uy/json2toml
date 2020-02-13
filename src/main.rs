use serde::Serialize;
use serde_json::{from_str, Value as JsonValue};
use std::{
    env::args,
    fmt::Display,
    fs::File,
    io::{prelude::*, stdin, stdout, BufReader, BufWriter},
    process::exit,
};
use toml::{Serializer, Value as TomlValue};

trait UnwrapOrExit {
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

fn main() {
    let arguments: Vec<_> = args().collect();
    if arguments.len() <= 2 {
        eprintln!("Usage: json2toml <input file> <output file>");
        exit(1);
    }

    let mut input_file: BufReader<Box<dyn Read>> = BufReader::new(match arguments.get(1) {
        Some(filename) => Box::new(File::open(filename).unwrap_or_exit("File error")),
        None => Box::new(stdin()),
    });

    let mut json = String::new();
    input_file
        .read_to_string(&mut json)
        .unwrap_or_exit("Invalid UTF-8 text");

    let json_value: JsonValue = from_str(&json).unwrap_or_exit("Invalid JSON");
    let toml_value = TomlValue::try_from(json_value).unwrap_or_exit("Can't convert to TOML");

    let mut toml_string = String::new();
    toml_value
        .serialize(&mut Serializer::new(&mut toml_string))
        .expect("Should be successful");

    let mut output_file: BufWriter<Box<dyn Write>> = BufWriter::new(match arguments.get(2) {
        Some(filename) => Box::new(File::create(filename).unwrap_or_exit("File error")),
        None => Box::new(stdout()),
    });
    output_file.write(toml_string.as_bytes()).unwrap();
    output_file.flush().unwrap();
}
