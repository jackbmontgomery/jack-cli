use home;
use std::fs::{DirBuilder, File, OpenOptions};
use std::io;
use std::io::Write;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub struct Storage {
    pub storage_file: File,
}

impl Storage {
    pub fn build() -> io::Result<Storage> {
        let mut storage_dir = Storage::find_home_dir()?;
        storage_dir.push(".cache/jack");

        DirBuilder::new().recursive(true).create(&storage_dir)?;

        let storage_file_path = &mut storage_dir;
        storage_file_path.push("storage.dat");

        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(storage_file_path)?;

        let storage = Storage { storage_file: file };

        Ok(storage)
    }

    fn find_home_dir() -> io::Result<PathBuf> {
        match home::home_dir() {
            None => {
                return Err(Error::new(
                    ErrorKind::NotFound,
                    "Unable to find home directory",
                ))
            }
            Some(file) => Ok(file),
        }
    }

    pub fn write(&mut self, data: &str) -> io::Result<()> {
        self.storage_file.write_all(data.as_bytes())
    }
}
