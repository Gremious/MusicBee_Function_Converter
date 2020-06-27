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
    // Open the file that we're going to read, in read/append mode. Create it if it doesn't exist.
    let path = format!("{}/files/mb_script.txt", env::current_dir()?.display());
    let mut open_file = OpenOptions::new().append(true).create(true).read(true).open(&path)?;

    // We're going to copy the file contents into a buffer String
    let mut buffer = String::new();
    open_file.read_to_string(&mut buffer).expect("Unable to read the buffer file");

    println!("Buffer: {}", buffer);

    let output_file = File::create(format!("{}/files/mb_script_output.txt", env::current_dir()?.display()))
        .expect("Unable to open file");
    let mut output_text = r#"$If(Criteria,True Result,False Result)"#.to_string();
    write_to_file(output_file, output_text);

    Ok(())
}

/*
// Thx stack overflow
use notify::{Watcher, RecursiveMode, RawEvent, op};

fn wait_until_file_created(file_path: PathBuf) -> Result<(), Box<Error>> {
    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::raw_watcher(tx).expect("something broke");
    // Watcher can't be registered for file that don't exists.
    // I use its parent directory instead, because I'm sure that it always exists
    let file_dir = file_path.parent().unwrap();
    watcher.watch(&file_dir, RecursiveMode::NonRecursive).expect("something broke");
    if !file_path.exists() {
        loop {
            match rx.recv_timeout(Duration::from_secs(2)).expect("something broke") {
                RawEvent { path: Some(p), op: Ok(op::CREATE), .. } =>
                    if p == file_path {
                        break
                    },
                _ => continue,
            }
        }
    }
    watcher.unwatch(file_dir).expect("something broke");
    Ok(())
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
