use std::env;

use serde_json;
use serde_json::{Map, Number, Value};

type RemainingEncodedString<'a> = &'a str;

// Custom implementation for decoding bencoded values.
fn decode_bencoded_value(mut encoded_value: &str) -> (Value, RemainingEncodedString) {
    let first_char = encoded_value.chars().next().unwrap();

    if first_char == 'd' {
        encoded_value = &encoded_value[1..(encoded_value.len() - 1)];

        let mut object_map: Map<String, Value> = Map::new();

        while encoded_value.len() > 0 {
            let (object_key, new_encoded_value) = decode_bencoded_value(encoded_value);
            encoded_value = new_encoded_value;

            match object_key {
                Value::String(key) => {
                    let (value, new_encoded_value) = decode_bencoded_value(encoded_value);
                    object_map.insert(key, value);
                    encoded_value = new_encoded_value;
                }
                _ => {}
            }
        }

        return (Value::Object(object_map), encoded_value);
    } else if first_char == 'l' {

        encoded_value = &encoded_value[1..(encoded_value.len() - 1)];

        let mut processed_chunks: Vec<Value> = vec![];

        while encoded_value.len() > 0 {
            let (processed_chunk, new_encoded_value) = decode_bencoded_value(encoded_value);
            processed_chunks.push(processed_chunk);
            encoded_value = new_encoded_value;
        }

        return (Value::Array(Vec::from(processed_chunks)), encoded_value);
    } else if first_char == 'i' {
        let first_e_index = encoded_value.find('e').unwrap();

        let (chunk, str_remainder) = encoded_value.split_at(first_e_index + 1);

        let number_value = chunk[1..(chunk.len() - 1)].parse::<i64>().unwrap();

        return (Value::Number(Number::from(number_value)), str_remainder);
    } else if first_char.is_digit(10) {
        let colon_index = encoded_value.find(':').unwrap();
        let string_len_number = &encoded_value[..colon_index].parse::<usize>().unwrap();

        let (chunk, str_remainder) = encoded_value.split_at(colon_index + 1 + string_len_number);

        let string_value = &chunk[colon_index + 1..chunk.len()];

        return (Value::String(string_value.to_string()), str_remainder);
    } else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let (decoded_value, _) = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
