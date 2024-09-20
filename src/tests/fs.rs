#[cfg(test)]
mod file {
  use std::{error::Error, fs::remove_dir_all, path::PathBuf};

  use crate::{fs::FileCreate, path::Directory};

  #[test]
  fn create_success() -> Result<(), Box<dyn Error>> {
    let content = "content";
    let file_create = FileCreate {
      content: String::from(content),
    };
    let path = PathBuf::from("test_file_success");
    Directory::create(&path)?;
    let result = file_create.new(&path, "file.txt");
    assert_eq!(result?, true);
    assert!(path.join("file.txt").exists());
    remove_dir_all(path)?;

    Ok(())
  }

  #[test]
  fn creation_failure() -> Result<(), Box<dyn Error>> {
    let content = "content";
    let file_create = FileCreate {
      content: String::from(content),
    };

    let path = PathBuf::from("test_file_failure");
    let filename = "invalid/name.txt"; // Name invalid

    Directory::create(&path)?;

    let result = std::panic::catch_unwind(|| {
      file_create.new(&path, filename).unwrap();
    });
    assert!(result.is_err());
    remove_dir_all(&path).unwrap();

    Ok(())
  }
}
