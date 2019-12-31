use std::fs::File;
use std::io::{BufReader, Read};
use std::io::{BufWriter, Write};
use crate::function_converter::file_helper;

pub fn writeToFile(mut input_file: File, input_text: String){
   input_file.write_all(input_text.as_bytes()).expect("Unable to write data");
}

