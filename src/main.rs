use crate::{reader::*, writer::*};
use std::{fs::File, io::Read};
pub mod reader;
pub mod writer;

extern crate rand;
use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn print_random_alphanum() -> String {
    let outputString = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect::<String>();
    outputString
}

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
