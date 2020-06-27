mod function_converter;
use crate::function_converter::reader::*;
use crate::function_converter::writer::*;
use std::fs;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut openFile    = File::open("src/files/textfile.txt").expect("Unable to open file");
    let mut createdFile = File::create("src/files/textfile2.txt").expect("Unable to open file");
    let mut file1Text = String::new();
    let mut file2Text = String::new();
    file2Text = "This is a test string".parse().unwrap();

    openFile.read_to_string(&mut file1Text).expect("Unable to read the file");
    println!("{}", file1Text);

    function_converter::writer::writeToFile(createdFile, file2Text);
}
