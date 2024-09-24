use std::env::{self, current_dir};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use relative_path::RelativePath;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    let relative_path = RelativePath::new(&args[2]);
    let root = current_dir().unwrap();
    let full_path = relative_path.to_path(&root);
    if command=="init"{
        let _ = init(Path::new(&full_path));
    }else if command=="check"{
        let res = check(Path::new(&full_path));
        println!("{res:?}");
    }
}

fn init(path:&Path)->io::Result<()>{
    let dir_iterator = fs::read_dir(path).unwrap_or_else(|error| {
        if error.kind() == io::ErrorKind::NotFound {
            panic!("Directory \"{path:?}\" not found");
        // } else if error.kind() == io::ErrorKind::NotADirectory {
        //     panic!("\"{path:?}\" is not a directory");
        } else if error.kind() == io::ErrorKind::PermissionDenied {
            panic!("Permission to \"{path:?}\" was denied");
        } else{
            panic!("Unknown file system problem: {error:?}");
        }
    });
    for entry in dir_iterator{
        let entry = match entry {
            Ok(entry)=>entry,
            Err(error)=>panic!("Unknown file system entry problem: {error:?}")
        };
        let entry_path = entry.path();
        if entry_path.is_dir() {
            init(&entry_path)?;
        } else {
            if entry_path.file_name().unwrap().to_str().map_or(false, |s| s.to_lowercase().ends_with(".ic")){
                continue;
            }
            let hash = hash_sum(&entry_path);
            let mut output = File::create(format!("{}.ic", entry_path.display().to_string()))?;
            write!(output, "{}", hash)?;
        }
    }
    return Ok(());
}

fn hash_sum(path:&Path)->u16{
    let mut result: u16 = 0;
    let first = true;
    let bytes = std::fs::read(path).unwrap_or_else(|error| {panic!("Error {error:?} with file {path:?}");});
    
    for byte_pair in bytes.chunks_exact(2) {
        if first{
            result = u16::from_le_bytes([byte_pair[0], byte_pair[1]]);
        }else{
            result = result ^ u16::from_le_bytes([byte_pair[0], byte_pair[1]]);
        }
    }

    return result;
}

fn check(path:&Path)->io::Result<Vec<PathBuf>>{
    let mut result:Vec<PathBuf> = Vec::new();
    
    let dir_iterator = fs::read_dir(path).unwrap_or_else(|error| {
        if error.kind() == io::ErrorKind::NotFound {
            panic!("Directory \"{path:?}\" not found");
        // } else if error.kind() == io::ErrorKind::NotADirectory {
        //     panic!("\"{path:?}\" is not a directory");
        } else if error.kind() == io::ErrorKind::PermissionDenied {
            panic!("Permission to \"{path:?}\" was denied");
        } else{
            panic!("Unknown file system problem: {error:?}");
        }
    });
    for entry in dir_iterator{
        let entry = match entry {
            Ok(entry)=>entry,
            Err(error)=>panic!("Unknown file system entry problem: {error:?}")
        };
        let entry_path = entry.path();
        if entry_path.is_dir() {
            let mut subdir_result = check(&entry_path)?;
            result.append(&mut subdir_result);
        } else {
            if entry_path.file_name().unwrap().to_str().map_or(false, |s| s.to_lowercase().ends_with(".ic")){
                continue;
            }
            let hash = hash_sum(&entry_path);
            let recorded = fs::read_to_string(format!("{}.ic", entry_path.display().to_string())).unwrap().to_string().parse::<u16>().unwrap();
            if hash != recorded{
                result.push(entry_path);
            }
        }
    }

    return Ok(result);
}