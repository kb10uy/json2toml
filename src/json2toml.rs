use serde::Serialize;
use serde_json::{from_str, Value as JsonValue};
use std::io::{prelude::*, BufReader, BufWriter};
use toml::{Serializer, Value as TomlValue};

use j2tlib::{inout_from_args, UnwrapOrExit};

fn main() {
    let (output, input) = inout_from_args().unwrap_or_exit("File error");
    let mut input_file = BufReader::new(input);
    let mut output_file = BufWriter::new(output);

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

    output_file.write(toml_string.as_bytes()).unwrap();
    output_file.flush().unwrap();
}
