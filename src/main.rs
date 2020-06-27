#![warn(clippy::all, clippy::pedantic)]
use crate::{reader::*, writer::*};
use std::{fs::File, io::Read};
pub mod reader;
pub mod writer;

fn main() {
    let mut openFile = File::open("src/files/textfile.txt").expect("Unable to open file");
    let mut createdFile = File::create("src/files/textfile2.txt").expect("Unable to open file");
    let mut file1Text = String::new();
    let mut file2Text = String::new();
    file2Text = "This is a test string".parse().unwrap();

    openFile
        .read_to_string(&mut file1Text)
        .expect("Unable to read the file");
    println!("{}", file1Text);

    writer::writeToFile(createdFile, file2Text);
}
