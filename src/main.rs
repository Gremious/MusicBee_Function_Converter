use std::fs;
use std::fs::File;
use std::io::Read;

pub mod function_converter {
    pub mod file_helper;
    pub mod reader;
    pub mod writer;
}

fn main() {
    let mut file1 = File::open("src/files/textfile.txt").expect("Unable to open file");
    let mut file2 = File::create("src/files/textfile2.txt").expect("Unable to open file");
    let mut file1Text = String::new();
    let mut file2Text = String::new();

    file1.read_to_string(&mut file1Text).expect("Unable to read the file");
    println!("{}", file1Text);

    function_converter::writer::writeToFile(file2, "text".parse().unwrap());
}
