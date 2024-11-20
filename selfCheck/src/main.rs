#[link_section = "check"]
#[used]
static mut HASH_SUM: u16 = 1234;

extern crate integrityCheckLib;

use memmap2::MmapOptions;
use object::{File, Object, ObjectSection};
use std::env;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};

fn get_section(file: &File, name: &str) -> Option<(u64, u64)> {
    for section in file.sections() {
        match section.name() {
            Ok(n) if n == name => {
                return section.file_range();
            }
            _ => {}
        }
    }
    None
}

fn hash_sum(entry_path: &Path, section_start: usize, section_end: usize) -> Option<u16> {
    if entry_path
        .file_name()?
        .to_str()
        .map_or(false, |s| s.to_lowercase().ends_with(".ic"))
    {
        return None;
    }
    let mut result: u16 = 0;
    // println!("{}", result);
    let mut first = true;
    let bytes = match std::fs::read(entry_path) {
        Ok(val) => val,
        Err(_) => return None,
    };

    let mut count: usize = 0;

    for byte_pair in bytes.chunks_exact(2) {
        if count >= section_start && count < section_end {
            continue;
        }

        // println!("{} {}", count, end);

        if first {
            result = u16::from_le_bytes([byte_pair[0], byte_pair[1]]);
            first = false;
            // println!("{}", result);
        } else {
            // let t = u16::from_le_bytes([byte_pair[0], byte_pair[1]]);
            // println!("{} {}", result, t);
            result = result ^ u16::from_le_bytes([byte_pair[0], byte_pair[1]]);
        }
        count = count + 2;
    }

    // println!("{}", result);
    return Some(result);
}

fn main() -> Result<(), Box<dyn Error>> {
    let prev_hash = unsafe { HASH_SUM };
    let file_name = env::current_exe()?;
    let tmp_name = file_name.with_extension("tmp");
    fs::copy(&file_name, &tmp_name)?;

    let file = OpenOptions::new().read(true).write(true).open(&tmp_name)?;
    let mut buf = unsafe { MmapOptions::new().map_mut(&file) }?;
    let file = File::parse(&*buf)?;

    if let Some(range) = get_section(&file, "check") {
        assert_eq!(range.1, 2);
        let base = range.0 as usize;
        let len = range.1 as usize;
        let file_buf = PathBuf::from(&file_name);
        let cur_hash = hash_sum(&file_buf, base, base + len).unwrap();

        // println!("{}", cur_hash);

        if prev_hash == 1234 {
            buf[base..(base + 2)].copy_from_slice(&(cur_hash).to_ne_bytes());

            let perms = fs::metadata(&file_name)?.permissions();
            fs::set_permissions(&tmp_name, perms)?;
            fs::rename(&tmp_name, &file_name)?;

            println!("It's the first run. The file is secure");
            return Ok(());
        }

        // println!("{} {}", cur_hash, prev_hash);

        if cur_hash != prev_hash {
            println!("The binary is compromized!!!");
            return Ok(());
        }

        println!("The binary is secure.");
    } else {
        fs::remove_file(&tmp_name)?;
        println!("Integrity check section of the file is missing. The binary is compromized!!!")
    }

    Ok(())
}
