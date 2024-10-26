use integrityCheckLib::{check, init};

use relative_path::RelativePath;
use std::env::{self, current_dir};
use std::path::Path;

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
