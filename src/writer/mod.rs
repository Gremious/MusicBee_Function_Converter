use std::fs::File;
use std::io::{BufReader, Read};
use std::io::{BufWriter, Write};

pub fn write_to_file(mut input_file: File, input_text: String){
   input_file.write_all(input_text.as_bytes()).expect("Unable to write data");
}

