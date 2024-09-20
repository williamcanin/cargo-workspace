use crate::utils::binary_name;
use colored::*;

pub struct Menu<'a> {
  pub args: &'a Vec<String>,
}

impl Menu<'_> {
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
            println!("Usage: {} new <PATH>", binary_name());
            return None;
          }
        }
        _ => {
          println!(
            "{} Unknown option: {}",
            " Warning: ".on_yellow().black(),
            self.args[i]
          );
          println!("Usage: {} new <PATH>", binary_name());
          return None;
        }
      }
    }
    opt
  }
}
