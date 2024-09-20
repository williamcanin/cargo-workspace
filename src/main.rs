mod constants;
mod fs;
mod git;
mod menu;
mod path;
mod tests;
mod utils;

use crate::menu::Menu;
use colored::*;
use fs::FileCreate;
use git::Git;
use path::Directory;
use std::{env, error::Error, path::Path, process::exit};
use utils::binary_name;

fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect();
  let menu = Menu { args: &args }.new();

  if menu.is_none() {
    exit(1);
  }

  match menu {
    Some(folder) => {
      let folder = folder.trim();

      // ------- Create path workspace -------
      Directory::create(Path::new(folder))?;

      // ------- Ignore if workspace already exists -------
      if Path::new(&folder).join("Cargo.toml").exists() {
        println!(
          "{} The workspace `{folder}` already exists",
          " Error: ".on_red()
        );
        exit(1);
      }

      // ------- Creates the workspace Cargo.toml -------
      FileCreate {
        content: String::from(constants::CARGO_TOML_CONTENT),
      }
      .new(&Path::new(&folder), "Cargo.toml")?;

      // ------- Creates the `cargo` configuration file -------
      FileCreate {
        content: String::from(constants::CARGO_CONFIG_CONTENT),
      }
      .new(&Path::new(&folder).join(".cargo"), "config.toml")?;

      // ------- Start `git init` in the workspace -------
      Git {
        content: String::from(constants::GIT_IGNORE_CONTENT),
      }
      .init(&Path::new(&folder))?;

      println!("    {} `{folder}` workspace skeleton", "Creating".green());
    }
    None => {
      println!(
        "{} Usage: {} new <PATH>",
        " Warning: ".on_yellow().black(),
        binary_name()
      );
      exit(1);
    }
  }

  Ok(())
}
