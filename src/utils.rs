pub fn binary_name() -> String {
  let file_path = std::env::current_exe().unwrap();
  let name = file_path.file_name().unwrap().to_str().unwrap().to_string();

  if let Some((part0, _)) = name.split_once('.') {
    part0.to_string()
  } else {
    name
  }
}
