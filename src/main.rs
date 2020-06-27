#![warn(clippy::all, clippy::pedantic)]
use crate::{reader::*, writer::*};
use std::{env, fs, fs::File, path::Path,
  io::{ Read, BufRead, BufReader, Error, ErrorKind }
};
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;
use std::thread::sleep;
use std::fs::OpenOptions;

pub mod reader;
pub mod writer;

fn main() -> std::io::Result<()> {
    let curr_dir = env::current_dir()?;
    // Open the file that we're going to read, in read/append mode. Create it if it doesn't exist.
    fs::create_dir_all(format!("{}/files/", curr_dir.display()))?;
    let path = format!("{}/files/mb_script.txt", curr_dir.display());
    let mut open_file = OpenOptions::new().append(true).create(true).read(true).open(&path)?;

    // We're going to copy the file contents into a buffer String
    let mut buffer = String::new();
    open_file.read_to_string(&mut buffer).expect("Unable to read the buffer file");

    println!("Buffer: {}", buffer);

    let output_file = File::create(format!("{}/files/mb_script_output.txt", curr_dir.display()))
        .expect("Unable to open file");
    let mut output_text = r#"$If(Criteria,True Result,False Result)"#.to_string();
    write_to_file(output_file, output_text);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
