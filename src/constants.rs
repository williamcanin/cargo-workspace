pub const CARGO_CONFIG_CONTENT: &str = r#"[build]
target-dir = "target"
target = "x86_64-pc-windows-msvc"

# Windows
[target.x86_64-pc-windows-msvc]
rustflags = [
    "-C", "link-arg=/MANIFEST:NO",
    "-C", "opt-level=3"
]

# Linux (GNU)
[target.x86_64-unknown-linux-gnu]
rustflags = [
    "-C", "opt-level=2",
    "-C", "target-feature=+avx2"
]"#;

pub const CARGO_TOML_CONTENT: &str = r#"[workspace]
resolver = "2"
members = []

# -------- Optimization --------
[profile.dev.package."*"]
opt-level = 0
incremental = true
debug = true

[profile.release.package."*"]
opt-level = 3
codegen-units = 1
debug = false"#;

pub const GIT_IGNORE_CONTENT: &str = r#"# Windows taken from:
# https://github.com/github/gitignore/blob/master/Global/Windows.gitignore
# -------------------------------------------------------------------------
Desktop.ini
ehthumbs.db
Thumbs.db
$RECYCLE.BIN/

# Linux
# -------------------------------------------------------------------------
*~

# VsCode
# -------------------------------------------------------------------------
/.vscode

# JetBrains
# -------------------------------------------------------------------------
/.idea

# Rust taken https://github.com/github/gitignore/blob/main/Rust.gitignore
# -------------------------------------------------------------------------
/target
Cargo.lock"#;
