#[cfg(test)]
mod run {
  use crate::menu::Menu;

  #[test]
  fn with_valid_argument() {
    let args = vec![
      String::from("cargo-workspace"),
      String::from("new"),
      String::from("project"),
    ];

    let menu_instance = Menu { args: &args };

    let result = menu_instance.new();
    assert_eq!(result, Some("project".to_string()));
  }

  #[test]
  fn without_path_argument() {
    let args = vec![String::from("cargo-workspace"), String::from("new")];
    let menu_instance = Menu { args: &args };
    let result = menu_instance.new();
    assert_eq!(result, None);
  }

  #[test]
  fn no_arguments() {
    let args = vec![String::from("cargo-workspace")];
    let menu_instance = Menu { args: &args };

    let result = menu_instance.new();
    assert_eq!(result, None);
  }

  #[test]
  fn unknown_option() {
    let args = vec![
      String::from("cargo-workspace"),
      String::from("unknown_option"),
    ];
    let menu_instance = Menu { args: &args };

    let result = menu_instance.new();
    assert_eq!(result, None);
  }
}
