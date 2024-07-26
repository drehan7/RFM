use std::{fs::{self, DirEntry}, io, path::Path};

// STYLES
pub const DIRECTORY_UNICODE:&str = "\u{1f4c1} ";
pub const FILE_UNICODE: &str = "\u{1f5ce} ";
pub const RIGHT_ARROW_UNICODE: &str = "\u{21E8} ";


pub fn get_dir_entries (path: &Path) -> Result<Vec<DirEntry>, io::Error> {
    match fs::read_dir(path) {
        Ok(entries) => {
            return entries
                .map(|m| m)
                .collect::<Result<Vec<DirEntry>, io::Error>>();
        }
        Err(e) => {
            return Err(e);
        }
    };
}

