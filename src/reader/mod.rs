use std::fs::File;
use std::io::{BufReader, Read};

pub fn read_file(input_file: File) -> String {
    let temp_file: File = input_file;
    let mut our_output_text = String::new(); // Sting we'll return

    let mut br = BufReader::new(temp_file);
    br.read_to_string(&mut our_output_text)
        .expect("Unable to read string");
    our_output_text
}

