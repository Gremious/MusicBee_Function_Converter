use std::fs::File;
use std::io::{BufReader, Read};
use std::io::{BufWriter, Write};

fn main() {
    let mainFile = File::open("src/textfile.txt").expect("Unable to open file");

    println!("{}", readFile(mainFile));
}

fn readFile(input_file: File) -> String {
    let tempFile: File = input_file;
    let mut ourOutputText = String::new(); // Sting we'll return

    let mut br = BufReader::new(tempFile);
    br.read_to_string(&mut ourOutputText)
        .expect("Unable to read string");
    ourOutputText
}
