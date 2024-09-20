use colored::Colorize;

use crate::fs::FileCreate;
use std::{error::Error, path::PathBuf, process::Command};

pub struct Git {
  pub content: String,
}

impl Git {
  fn create_gitignore(&self, path: &PathBuf, content: String) -> Result<(), Box<dyn Error>> {
    FileCreate { content }.new(path, ".gitignore")?;
    Ok(())
  }

  fn is_installed(&self) -> bool {
    let git_check = Command::new("git").arg("--version").output();

    match git_check {
      Ok(output) => match output.status.success() {
        true => true,
        false => false,
      },
      Err(_) => false,
    }
  }

  pub fn init(&self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
    if !self.is_installed() {
      return Ok(());
    }

    let git_init = Command::new("git").arg("init").current_dir(path).output();

    match git_init {
      Ok(output) => {
        if output.status.success() {
          self.create_gitignore(path, self.content.clone())?;
        }
      }
      Err(err) => {
        println!(
          "{} Failed to execute `git init` command: {}",
          " Error: ".on_red(),
          err
        );
      }
    }

    Ok(())
  }
}
