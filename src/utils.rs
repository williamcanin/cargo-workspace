pub fn binary_name() -> String {
  let bin_path = std::env::current_exe().unwrap();
  let bin_name = bin_path.file_name().unwrap().to_str().unwrap().to_string();
  bin_name.split_once(".").unwrap().0.to_string()
}
