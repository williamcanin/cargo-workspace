# Cargo Workspace

[![Windows Build](https://github.com/williamcanin/cargo-workspace/actions/workflows/windows.yml/badge.svg)](https://github.com/williamcanin/cargo-workspace/actions/workflows/windows.yml)
[![Linux Build](https://github.com/williamcanin/cargo-workspace/actions/workflows/linux.yml/badge.svg)](https://github.com/williamcanin/cargo-workspace/actions/workflows/linux.yml)
------------

**Cargo Workspace** is a project for creating workspaces in [Rust](https://www.rust-lang.org/) projects.
With **Cargo Workspace** you avoid manually creating files such as `Cargo.toml` and `.cargo/config.toml`.

## Features

* Create the `Cargo.toml` file, and fill it with basic configuration.
* Creates the job configuration file (`.cargo/config.toml`), and fills it with basic configuration.
* Do `git init` if Git is installed.
* Creates a basic `.gitignore`.

## Usage

You can clone the repository and compile the project if you have Rust installed, or download the already compiled binary from [releases](https://github.com/williamcanin/cargo-workspace/releases).
Place the compiled binary in a system PATH.

## Development

> NOTE: If you want to use and contribute to this project, please be aware of the [LICENSE](https://github.com/williamcanin/cargo-workspace/blob/main/LICENSE).

For development, there are some dependencies that you need to install.

**Requirements:**

| Required        | Required version    |   How to verify     | How to install                   |
| --------------- | ------------------- | ------------------- | -------------------------------- |
| make            |   >= 4.3.0          | `make --version`    | **Windows:** choco install make  |
| nightly         |  indifferent        | `rustup show`       | rustup install nightly           |

The `Makefile` file is intended to group commands from `cargo` and `git`.

The `nightly` will be used to format the code through the `rustfmt.toml` file.

**Build:**

1 - Git clone:

```
git clone https://github.com/williamcanin/cargo-workspace.git
```

2 - Compile:

```
cargo build
```

------
(c) Copyright 2024 - William Canin - All rights reserved
