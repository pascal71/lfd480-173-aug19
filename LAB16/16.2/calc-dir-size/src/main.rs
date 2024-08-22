use std::env;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
use std::time::SystemTime;

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<u64> {
    let mut total_size = 0;
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                total_size += visit_dirs(&path, cb)?;
            } else {
                total_size += entry.metadata()?.len();
                cb(&entry);
            }
        }
    }
    Ok(total_size)
}

fn main() {
    let current_dir = env::current_dir().expect("Failed to determine current directory");
    let total_size = visit_dirs(&current_dir, &|entry: &DirEntry| {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "rs") {
            println!("Rust file: {:?}", path);
            let metadata = entry.metadata().expect("Failed to get file metadata");
            let modified_time = metadata
                .modified()
                .expect("Failed to get modification time");
            let duration = modified_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Time went backwards");
            println!("Last modified: {:?}", duration);
        }
    })
    .expect("Failed to traverse directories");
    println!("Total directory size: {} bytes", total_size);
}
