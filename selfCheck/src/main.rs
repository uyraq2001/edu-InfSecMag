use std::env::{self, current_dir};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[0];
    let lockname = filename.clone().replace(".exe", ".ic");
    let lockpath = Path::new(&filename);

    let hash = hash_sum(lockpath);
    let recorded = match fs::read_to_string(Path::new(&lockname)) {
        Ok(res) => res.to_string().parse::<u16>().unwrap(),
        Err(_) => panic!("File have been modified!!!"),
    };
    if hash == recorded {
        println!("File is secure");
        return;
    }
    if recorded == 1234 {
        println!("It's a first run. File is secure");
        write!(File::create(lockpath).unwrap(), "{}", hash);
        return;
    }
}

fn hash_sum(path: &Path) -> u16 {
    let mut result: u16 = 0;
    let first = true;
    let bytes = std::fs::read(path).unwrap_or_else(|error| {
        panic!("Error {error:?} with file {path:?}");
    });

    for byte_pair in bytes.chunks_exact(2) {
        if first {
            result = u16::from_le_bytes([byte_pair[0], byte_pair[1]]);
        } else {
            result = result ^ u16::from_le_bytes([byte_pair[0], byte_pair[1]]);
        }
    }

    return result;
}
