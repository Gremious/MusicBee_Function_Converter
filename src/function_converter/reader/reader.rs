use std::fs::File;
use std::io::{BufReader, Read};

pub fn read_file(input_file: File) -> String {
    let tempFile: File = input_file;
    let mut ourOutputText = String::new(); // Sting we'll return

    let mut br = BufReader::new(tempFile);
    br.read_to_string(&mut ourOutputText)
        .expect("Unable to read string");
    ourOutputText
}
