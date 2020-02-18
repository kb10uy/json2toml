use serde::Serialize;
use std::io::{prelude::*, BufReader, BufWriter};
use toml::{Serializer, Value};

use j2tlib::{inout_from_args, UnwrapOrExit};

fn main() {
    let (output, input) = inout_from_args().unwrap_or_exit("File error");
    let mut input_file = BufReader::new(input);
    let mut output_file = BufWriter::new(output);

    let mut toml_string = String::new();
    input_file
        .read_to_string(&mut toml_string)
        .unwrap_or_exit("Invalid file");
    let toml_value: Value = toml::from_str(&toml_string).unwrap_or_exit("Invalid TOML");
    let toml_schema = value_to_schema(toml_value);

    let mut toml_string = String::new();
    toml_schema
        .serialize(&mut Serializer::new(&mut toml_string))
        .expect("Should be successful");

    output_file.write(toml_string.as_bytes()).unwrap();
    output_file.flush().unwrap();
}

fn value_to_schema(toml: Value) -> Value {
    match toml {
        Value::Array(_) => Value::String("array".into()),
        Value::Boolean(_) => Value::String("boolean".into()),
        Value::Datetime(_) => Value::String("datetime".into()),
        Value::Float(_) => Value::String("float".into()),
        Value::Integer(_) => Value::String("integer".into()),
        Value::String(_) => Value::String("string".into()),
        Value::Table(table) => Value::Table(
            table
                .into_iter()
                .map(|(key, value)| (key, value_to_schema(value)))
                .collect(),
        ),
    }
}
