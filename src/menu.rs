use std::process::exit;

use crate::utils::binary_name;
use colored::*;

pub struct Menu {
  pub args: Vec<String>,
}

impl Menu {
  pub fn help(&self) {
    println!("Usage: {} <command> <path> ...", binary_name());
    println!("\n{}", "Commands:".bright_green());
    println!(
      "  {}                   Create new workspace.",
      "new".bright_cyan()
    );
    println!(
      "  {}             Print this message and exit.\n",
      "-h,--help".bright_cyan()
    );
    println!("(C) Copyright 2024 - Cargo Workspace - All right reserved.");
    exit(0);
  }
  pub fn new(&self) -> Option<String> {
    let mut opt = None;
    let mut i = 1;
    while i < self.args.len() {
      match self.args[i].as_str() {
        "new" => {
          if i + 1 < self.args.len() {
            opt = Some(self.args[i + 1].clone());
            i += 2;
          } else {
            println!("{} 'new' requires a <PATH> argument", " Error: ".on_red());
            return None;
          }
        }
        "--help" | "-h" | "help" => self.help(),
        _ => {
          println!(
            "{} Unknown option: {}",
            " Warning: ".on_yellow().black(),
            self.args[i]
          );
          return None;
        }
      }
    }
    opt
  }
}
