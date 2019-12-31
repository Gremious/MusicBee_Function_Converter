use std::fs::File;

pub mod function_converter {
    mod file_helper {
        pub mod file_helper;
    }
    pub mod reader {
        pub mod reader;
    }
    pub mod writer {
        pub mod writer;
    }
}

fn main() {
    let mainFile = File::open("src/files/textfile.txt").expect("Unable to open file");
    let file2 = File::create("src/files/textfile2.txt").expect("Unable to open file");
    println!("{}", function_converter::reader::reader::read_file(mainFile));
    function_converter::writer::writer::writeToFile(file2, "text".parse().unwrap());
}
