# SakuraMML Build Makefile

# Variables
TARGET_DIR = ./sakuramml-bin
TARGET_ZIP = ./mac-sakuramml-bin.zip
SCRIPT_DIR = $(PWD)

# Default target
.PHONY: all build bin wasm doc doc-check clean fmt fmt-check help

all: build

# Build release binary
build:
	make wasm
	make doc
	cargo build --release

# Build binary distribution package
bin: build
	mkdir -p $(TARGET_DIR)
	cp README.md $(TARGET_DIR)/
	cp README_ja.md $(TARGET_DIR)/
	cp target/release/sakuramml $(TARGET_DIR)/
	zip $(TARGET_ZIP) -r $(TARGET_DIR)
	@echo "Binary package created: $(TARGET_ZIP)"

# Build WebAssembly
wasm:
	cnako3 $(SCRIPT_DIR)/src/batch_version.nako3
	wasm-pack build --target web
	cnako3 $(SCRIPT_DIR)/update_version.nako3
	@echo "WASM build completed"

# Build documentation
doc:
	@echo "--- build doc ---"
	python3 $(SCRIPT_DIR)/scripts/extract_command.py
	@echo "Documentation build completed"

# Verify generated documentation
doc-check:
	python3 $(SCRIPT_DIR)/scripts/extract_command.py --check

# Clean build artifacts
clean:
	cargo clean
	rm -rf $(TARGET_DIR)
	rm -f $(TARGET_ZIP)
	rm -rf pkg/

# Debug build
debug:
	cargo build

# Run tests
test:
	python3 -m unittest discover -s tests -p 'test_*.py'
	cargo test

# Format code
fmt:
	cargo fmt --all

# Verify code formatting
fmt-check:
	cargo fmt --all -- --check

# Run clippy
clippy:
	cargo clippy

# Install dependencies (requires Python 3 and cnako3 for WASM builds)
deps:
	@echo "Please ensure Python 3 is installed for doc builds"
	@echo "Please ensure cnako3 is installed for WASM builds"

# Help
help:
	@echo "Available targets:"
	@echo "  all       - Default target, builds release binary"
	@echo "  build     - Build release binary"
	@echo "  bin       - Build binary distribution package"
	@echo "  wasm      - Build WebAssembly version"
	@echo "  doc       - Build documentation"
	@echo "  doc-check - Verify generated documentation"
	@echo "  debug     - Build debug version"
	@echo "  test      - Run tests"
	@echo "  fmt       - Format all Rust code with cargo fmt"
	@echo "  fmt-check - Verify Rust code formatting"
	@echo "  clippy    - Run clippy linter"
	@echo "  clean     - Clean build artifacts"
	@echo "  deps      - Show dependency information"
	@echo "  help      - Show this help message"
