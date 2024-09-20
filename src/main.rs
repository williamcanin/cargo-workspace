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
use std::{env, error::Error, path::PathBuf, process::exit};
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
      let root = PathBuf::from(folder);

      // ------- Ignore if workspace already exists -------
      if root.join("Cargo.toml").exists() {
        println!(
          "{} The workspace `{folder}` already exists",
          " Error: ".on_red()
        );
        exit(1);
      }

      // // ------- Create root workspace -------
      // Directory::create(&root)?;

      // ------- Creates the workspace Cargo.toml -------
      FileCreate {
        content: String::from(constants::CARGO_TOML_CONTENT),
      }
      .new(&root, "Cargo.toml")?;

      // // ------- Create .cargo folder -------
      // Directory::create(&root.join(".cargo"))?;

      // ------- Creates the `cargo` configuration file -------
      FileCreate {
        content: String::from(constants::CARGO_CONFIG_CONTENT),
      }
      .new(&root.join(".cargo"), "config.toml")?;

      // ------- Start `git init` in the workspace -------
      Git {
        content: String::from(constants::GIT_IGNORE_CONTENT),
      }
      .init(&root)?;

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
