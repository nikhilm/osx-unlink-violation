extern crate libc;

use std::fs::{DirEntry, OpenOptions};
use std::path::Path;
use std::os::unix::fs::OpenOptionsExt;
use std::os::macos::fs::MetadataExt;
use std::io::{Result, Read};

fn link_count<P: AsRef<Path>>(path: P) -> Result<u64> {
    std::fs::metadata(path).map(|x| x.st_nlink())
}

fn link_count_file(f: std::fs::File) -> Result<u64> {
    f.metadata().map(|x| x.st_nlink())
}

fn main() {
    let pid = unsafe { libc::getpid() };
    let dir_iter: Vec<Result<DirEntry>> = std::path::Path::new(".")
        .read_dir()
        .expect("read dir failed")
        .collect();
    std::thread::sleep(std::time::Duration::from_millis(10));
    for entry in dir_iter {
        if let Ok(entry) = entry {
            if !entry
                    .path()
                    .to_str()
                    .expect("utf8")
                    .ends_with(".sample") {
                continue;
            }
            let mut file = std::fs::OpenOptions::new()
                .read(true)
                .custom_flags(0x20 /* EXLOCK */ | libc::O_NONBLOCK)
                .open(entry.path());
            match file {
                Ok(_) => {
                    println!("{} Opened {:?}", pid, entry.path());
                }
                Err(err) => {
                    println!("{} {:?} {:?}", pid, entry.path(), err);
                    continue;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(2));
            println!("{} Link count {:?} {:?}",
                     pid,
                     entry.path(),
                     link_count(entry.path()));
            match std::fs::remove_file(entry.path()) {
                Ok(_) => {}
                Err(err) => {
                    println!("!!!!! {} Could not remove {:?} {:?}",
                             pid,
                             entry.path(),
                             err);
                }
            }
            println!("{} Link count after removal {:?} {:?}",
                     pid,
                     entry.path(),
                     link_count_file(file.unwrap()));
        }
    }
}
