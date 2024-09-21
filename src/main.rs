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
use std::{env, error::Error, path::PathBuf, process::exit};

fn main() -> Result<(), Box<dyn Error>> {
  let menu = Menu {
    args: env::args().collect(),
  };

  match menu.new() {
    Some(folder) => {
      let folder = folder.trim();
      let root = PathBuf::from(folder);

      // ------- Create root workspace -------
      if let Err(e) = Directory::create(&root) {
        println!("{}", e);
        exit(1);
      }

      // ------- Creates the workspace Cargo.toml -------
      let instance = FileCreate {
        content: String::from(constants::CARGO_TOML_CONTENT),
      };
      if let Err(e) = instance.new(&root, "Cargo.toml") {
        println!("{}", e);
        exit(1);
      }

      // ------- Create .cargo folder -------
      if let Err(e) = Directory::create(&root.join(".cargo")) {
        println!("{}", e);
        exit(1);
      }

      // ------- Creates the `cargo` configuration file -------
      let instance = FileCreate {
        content: String::from(constants::CARGO_CONFIG_CONTENT),
      };
      if let Err(e) = instance.new(&root.join(".cargo"), "config.toml") {
        println!("{}", e);
        exit(1);
      }

      // ------- Start `git init` in the workspace -------
      let instance = Git {
        content: String::from(constants::GIT_IGNORE_CONTENT),
      };
      if let Err(e) = instance.init(&root) {
        println!("{}", e);
        exit(1);
      };

      println!("    {} `{folder}` workspace skeleton", "Creating".green());
    }
    None => {
      menu.help();
    }
  }

  Ok(())
}
