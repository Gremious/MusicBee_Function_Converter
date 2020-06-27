#![warn(clippy::all, clippy::pedantic)]
use crate::{reader::*, writer::*};
use std::{fs::File, io::Read};
pub mod reader;
pub mod writer;

fn main() {
    let mut open_file = File::open("src/files/textfile.txt").expect("Unable to open file");
    let created_file = File::create("src/files/textfile2.txt").expect("Unable to open file");
    let mut file1text = String::new();
    let mut file2text = String::new();
    file2text = "This is a test string".parse().unwrap();

    open_file
        .read_to_string(&mut file1text)
        .expect("Unable to read the file");
    println!("{}", file1text);

    writer::write_to_file(created_file, file2text);
}
