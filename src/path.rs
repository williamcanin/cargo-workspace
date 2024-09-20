use colored::Colorize;
use std::{error::Error, fs::create_dir_all, path::Path};

pub struct Directory;

impl Directory {
  pub fn create(path: &Path) -> Result<bool, Box<dyn Error>> {
    if !path.exists() {
      match create_dir_all(path) {
        Ok(_) => Ok(true),
        Err(e) => match e.raw_os_error() {
          Some(123) => Err(
            format!(
              "{} Invalid file name or characters `{}` not allowed",
              " Error: ".on_red(),
              path.to_str().unwrap_or("")
            )
            .into(),
          ),
          Some(code) => {
            Err(format!("{} Unexpected failure. Code: {}", " Error: ".on_red(), code).into())
          }
          None => Err(format!("{} Unexpected failure. Debug: {}", " Error: ".on_red(), e).into()),
        },
      }
    } else {
      Err(
        format!(
          "{} Directory `{}` already exists",
          " Error: ".on_red(),
          path.to_str().unwrap()
        )
        .into(),
      )
    }
  }
}
