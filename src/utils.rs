use std::{fs::{self, DirEntry}, io, path::Path};

#[warn(dead_code)]
pub fn list_files(path: &Path) -> Result<(),()> { 
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        if let Ok(metadata) = entry.metadata() {
                            println!("{:?} {:?}", entry.path(), metadata);
                        }
                    }
                    Err(err) => eprintln!("Error: {}", err),
                }
            }
        },
        Err(e) => eprintln!("Error: {}", e)
    }

    Ok(())
}

pub fn get_dir_entries (path: &Path) -> Result<Vec<DirEntry>, io::Error> {
    match fs::read_dir(path) {
        Ok(entries) => {
            // for entry in entries {
            //     println!("{:?}", entry?.path());
            // }
            // return Ok(Vec::new());
            return entries
                .map(|m| m)
                .collect::<Result<Vec<DirEntry>, io::Error>>();
        }
        Err(e) => {
            return Err(e);
        }
    };
}

