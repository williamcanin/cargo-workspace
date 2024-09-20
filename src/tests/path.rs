#[cfg(test)]
mod directory {
  use colored::Colorize;

  use crate::path::Directory;
  use std::fs::{create_dir_all, remove_dir_all};
  use std::path::PathBuf;

  #[test]
  fn create_success() {
    let path = PathBuf::from("test_dir_success");
    if path.exists() {
      remove_dir_all(&path).unwrap();
    }
    assert!(Directory::create(&path).unwrap());
    assert!(path.exists());
    remove_dir_all(&path).unwrap();
  }

  #[test]
  fn create_exists() {
    let path = PathBuf::from("test_dir_exists");
    if path.exists() {
      remove_dir_all(&path).unwrap();
    }
    create_dir_all(&path).unwrap();
    let result = Directory::create(&path);

    // match result {
    //   Ok(value) => match value {
    //     true => assert!(false),
    //     false => assert!(true),
    //   },
    //   Err(_) => assert!(false),
    // }

    assert!(result.is_err());
    assert_eq!(
      result.unwrap_err().to_string(),
      format!(
        "{} Directory `{}` already exists",
        " Error: ".on_red(),
        path.to_str().unwrap()
      )
    );
    remove_dir_all(&path).unwrap();
  }

  #[test]
  fn create_invalid_name() {
    let path = PathBuf::from("test_dir/invalid\0name");
    // let path = PathBuf::from("test_dir_invalid*<");
    let result = std::panic::catch_unwind(|| {
      Directory::create(&path).unwrap();
    });
    assert!(result.is_err());
  }
}
