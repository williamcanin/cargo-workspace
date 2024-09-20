# File type: Makefile
# Author: William C. Canin <https://williamcanin.github.io>

# Install Make in Windows:
# >>> choco install make

commands:
	@echo Commands:
	@echo     make commit ---------- Branch commit (using git)
	@echo     make push ------------ Push your project for repository
	@echo     make fmt ------------- Format code
	@echo     make tests ----------- Tests
	@echo     make build ----------- Compile code
	@echo     make release --------- Compile code for release
	@echo     make clean ----------- Delete entire build directory
	@echo ----------------------------------------------------------
	@echo     (c) 2024 - William C. Canin - Makefile commands

fmt:
	@cargo +nightly fmt

tests:
	@cargo test

push: fmt
	@git push origin
	@git push hdd

build:
	@cargo build

release:
	@cargo build --release

clean:
	@cargo clean
