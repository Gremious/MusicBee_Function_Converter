use std::fs::File;
use std::io::{BufReader, Read};
use std::io::{BufWriter, Write};
use reader;
use writer;
use fileHelper;

fn main() {
    let mainFile = File::open("src/files/textfile.txt").expect("Unable to open file");
    println!("{}", reader::readFile(mainFile));
}
