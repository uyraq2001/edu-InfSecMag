use regex::Regex;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub enum CheckResult {
    Removed,
    Added,
    Changed,
    Secure,
}

pub fn hash_sum(entry_path: &Path) -> Option<u16> {
    if entry_path
        .file_name()?
        .to_str()
        .map_or(false, |s| s.to_lowercase().ends_with(".ic"))
    {
        return None;
    }
    let mut result: u16 = 0;
    let mut first = true;
    let bytes = match std::fs::read(entry_path) {
        Ok(val) => val,
        Err(_) => return None,
    };

    for byte_pair in bytes.chunks_exact(2) {
        if first {
            result = u16::from_le_bytes([byte_pair[0], byte_pair[1]]);
            first = false;
        } else {
            result = result ^ u16::from_le_bytes([byte_pair[0], byte_pair[1]]);
        }
    }

    return Some(result);
}

pub fn apply_to_dir<F>(path: &Path, mut func: F)
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

pub fn check(path: &Path) -> io::Result<(Vec<PathBuf>, Vec<PathBuf>, Vec<PathBuf>)> {
    let mut changed: Vec<PathBuf> = Vec::new();
    let mut removed: Vec<PathBuf> = Vec::new();
    let mut added: Vec<PathBuf> = Vec::new();
    apply_to_dir(path, |entry_path: PathBuf| -> io::Result<()> {
        if entry_path.is_dir() {
            let mut subdir_result = check(&entry_path)?;
            changed.append(&mut subdir_result.0);
            removed.append(&mut subdir_result.1);
            added.append(&mut subdir_result.2);
            return Ok(());
        }
        match check_single(&entry_path)? {
            CheckResult::Changed => changed.push(get_file_name_for_result(&entry_path)),
            CheckResult::Removed => removed.push(get_file_name_for_result(&entry_path)),
            CheckResult::Added => added.push(get_file_name_for_result(&entry_path)),
            CheckResult::Secure => {}
        }
        return Ok(());
    });
    return Ok((changed, removed, added));
}

pub fn init(path: &Path) -> io::Result<()> {
    clear_ic_files(path)?;
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

pub fn read_hash(entry_path: &Path) -> Option<u16> {
    let hash_str = match fs::read_to_string(entry_path) {
        Ok(val) => val,
        Err(_) => return None,
    };
    match hash_str.parse::<u16>() {
        Ok(val) => return Some(val),
        Err(_) => return None,
    }
}

pub fn check_single(entry_path: &PathBuf) -> io::Result<CheckResult> {
    let mut hash: u16 = 0;
    let mut recorded: u16 = 0;

    let file_name = match entry_path.file_name() {
        Some(val) => val,
        None => return Err(io::Error::new(io::ErrorKind::NotFound, "File not found")),
    };
    if !file_name
        .to_str()
        .map_or(false, |s| s.to_lowercase().ends_with(".ic"))
    {
        hash = match hash_sum(&entry_path) {
            Some(val) => val,
            None => return Err(io::Error::new(io::ErrorKind::NotFound, "File not found")),
        };
        let hash_path = format!("{}.ic", entry_path.display().to_string());
        recorded = match read_hash(Path::new(&hash_path)) {
            Some(val) => val,
            None => return Ok(CheckResult::Added),
        };
    } else {
        let mut orig_path = entry_path.display().to_string();
        orig_path.truncate(orig_path.len() - 3);
        hash = match hash_sum(Path::new(&orig_path)) {
            Some(val) => val,
            None => return Ok(CheckResult::Removed),
        };
        recorded = match read_hash(&entry_path) {
            Some(val) => val,
            None => return Err(io::Error::new(io::ErrorKind::NotFound, "File not found")),
        };
        return Ok(CheckResult::Secure);
    }
    if hash != recorded {
        return Ok(CheckResult::Changed);
    }
    return Ok(CheckResult::Secure);
}

pub fn init_single(entry_path: &PathBuf) -> io::Result<()> {
    let hash = match hash_sum(&entry_path) {
        Some(val) => val,
        None => return Ok(()),
    };
    let mut output = File::create(format!("{}.ic", entry_path.display().to_string()))?;
    write!(output, "{}", hash)?;
    return Ok(());
}

pub fn clear_ic_files(path: &Path) -> io::Result<()> {
    apply_to_dir(path, |entry_path: PathBuf| -> io::Result<()> {
        let file_name = match entry_path.to_str() {
            Some(val) => val,
            None => return Err(io::Error::new(io::ErrorKind::NotFound, "File not found")),
        };
        if file_name.to_lowercase().ends_with(".ic") {
            fs::remove_file(file_name)?;
        }
        return Ok(());
    });
    return Ok(());
}

fn get_file_name_for_result(entry_path: &PathBuf) -> PathBuf {
    let entry_str = entry_path.to_str().unwrap();
    let re = Regex::new(r"(.*)\.ic").unwrap();
    match re.captures(entry_str) {
        Some(caps) if caps.len() >= 2 => {
            return PathBuf::from(caps.get(1).map_or("", |m| m.as_str()))
        }
        _ => return PathBuf::from(entry_str),
    };
}
