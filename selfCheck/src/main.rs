extern crate integrityCheckLib;

use core::error;
use std::env::{self, current_dir};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[0];
    let lock_name = format!("{}.ic", file_name);
    let lock_path = Path::new(&lock_name);

    let recorded = match integrityCheckLib::read_hash(Path::new(&lock_path)) {
        Some(val) => val,
        None => {
            println!("The integrity check file is missing!!!");
            return;
        }
    };
    let file_path = PathBuf::from(file_name);

    if recorded == 1234 {
        match integrityCheckLib::init_single(&file_path) {
            Err(error) => panic!("{error:?}"),
            _ => {}
        }
        println!("It's the first run. The file is secured");
        return;
    }

    let hash = integrityCheckLib::hash_sum(&file_path).unwrap();
    if hash != recorded {
        println!("The binary is compromized!!!");
        return;
    }
    println!("File is secure");
}
