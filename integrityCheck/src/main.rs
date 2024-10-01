use relative_path::RelativePath;
use std::env::{self, current_dir};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    let relative_path = RelativePath::new(&args[2]);
    let root = current_dir().unwrap_or_else(|error| {
        panic!("Attempt to get current dir failed with error: \n{error:?}");
    });
    let full_path = relative_path.to_path(&root);
    if command == "init" {
        let _ = init(Path::new(&full_path));
        println!("Dirrectory {full_path:?} have been initialized")
    } else if args.len() > 2 && command == "check" {
        let res = check(Path::new(&full_path)).unwrap();
        if res.len() == 0 {
            println!("Dirrectory {full_path:?} have been checked.No files have been changed.");
        } else {
            println!("Dirrectory {full_path:?} have been checked. The following files have been changed:\n{res:?}");
        }
    }
}

fn hash_sum(entry_path: &Path) -> Option<u16> {
    if entry_path
        .file_name()?
        .to_str()
        .map_or(false, |s| s.to_lowercase().ends_with(".ic"))
    {
        return None;
    }
    let mut result: u16 = 0;
    let first = true;
    let bytes = std::fs::read(entry_path).unwrap_or_else(|error| {
        panic!("Error {error:?} with file {entry_path:?}");
    });

    for byte_pair in bytes.chunks_exact(2) {
        if first {
            result = u16::from_le_bytes([byte_pair[0], byte_pair[1]]);
        } else {
            result = result ^ u16::from_le_bytes([byte_pair[0], byte_pair[1]]);
        }
    }

    return Some(result);
}

fn apply_to_dir<F>(path: &Path, mut func: F)
where
    F: FnMut(PathBuf) -> io::Result<()>,
{
    let dir_iterator = fs::read_dir(path).unwrap_or_else(|error| {
        panic!("File system error: \n{error:?}\n for file: \"{path:?}\".");
    });
    for entry in dir_iterator {
        let entry = entry.unwrap_or_else(|error| {
            panic!("File system error: \n{error:?}");
        });
        let entry_path = entry.path();
        _ = func(entry_path);
    }
}

fn check(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut result: Vec<PathBuf> = Vec::new();
    apply_to_dir(path, |entry_path: PathBuf| -> io::Result<()> {
        if entry_path.is_dir() {
            let mut subdir_result = check(&entry_path)?;
            result.append(&mut subdir_result);
            return Ok(());
        }
        if !check_single(&entry_path)? {
            result.push(entry_path);
        }
        return Ok(());
    });
    return Ok(result);
}

fn init(path: &Path) -> io::Result<()> {
    apply_to_dir(path, |entry_path: PathBuf| -> io::Result<()> {
        if entry_path.is_dir() {
            init(&entry_path)?;
        } else {
            init_single(&entry_path)?;
        }
        return Ok(());
    });
    return Ok(());
}

fn read_hash(entry_path: &Path) -> Option<u16> {
    let hash_str = match fs::read_to_string(format!("{}.ic", entry_path.display().to_string())) {
        Ok(val) => val,
        Err(_) => return None,
    };
    match hash_str.parse::<u16>() {
        Ok(val) => return Some(val),
        Err(_) => return None,
    }
}

fn check_single(entry_path: &PathBuf) -> io::Result<bool> {
    let hash = match hash_sum(&entry_path) {
        Some(val) => val,
        None => return Ok(false),
    };
    let recorded = match read_hash(&entry_path) {
        Some(val) => val,
        None => return Ok(false),
    };
    if hash != recorded {
        return Ok(false);
    }
    return Ok(true);
}

fn init_single(entry_path: &PathBuf) -> io::Result<()> {
    let hash = match hash_sum(&entry_path) {
        Some(val) => val,
        None => return Ok(()),
    };
    let mut output = File::create(format!("{}.ic", entry_path.display().to_string()))?;
    write!(output, "{}", hash)?;
    return Ok(());
}
