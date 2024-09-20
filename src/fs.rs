use crate::path::Directory;
use colored::Colorize;
use std::{error::Error, fs::File, io::Write, path::Path};

pub struct FileCreate {
  pub content: String,
}

impl FileCreate {
  pub fn new(&self, path: &Path, filename: &str) -> Result<bool, Box<dyn Error>> {
    Directory::create(path)?;
    match File::create(path.join(filename)) {
      Ok(mut file) => match file.write_all(self.content.as_bytes()) {
        Ok(_) => Ok(true),
        Err(_) => panic!(
          "{} Failed to write to {} file",
          " Error: ".on_red(),
          path.join(filename).to_str().unwrap()
        ),
      },
      Err(_) => panic!(
        "{} Failed to create {}",
        " Error: ".on_red(),
        path.join(filename).to_str().unwrap()
      ),
    }
  }
}
