use colored::Colorize;
use std::{error::Error, fs::File, io::Write, path::PathBuf};
// use crate::path::Directory;

pub struct FileCreate {
  pub content: String,
}

impl FileCreate {
  pub fn new(&self, path: &PathBuf, filename: &str) -> Result<bool, Box<dyn Error>> {
    // if !Directory::create(path)? {
    //   return Ok(false);
    // }

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
