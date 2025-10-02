# SakuraMML Build Makefile

# Variables
TARGET_DIR = ./sakuramml-bin
TARGET_ZIP = ./mac-sakuramml-bin.zip
SCRIPT_DIR = $(PWD)

# Default target
.PHONY: all build bin wasm doc clean help

all: build

# Build release binary
build:
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
	cd $(SCRIPT_DIR)/src && cnako3 batch_extract_command.nako3
	@echo "Documentation build completed"

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
	cargo test

# Format code
fmt:
	cargo fmt

# Run clippy
clippy:
	cargo clippy

# Install dependencies (requires cnako3)
deps:
	@echo "Please ensure cnako3 is installed for WASM and doc builds"

# Help
help:
	@echo "Available targets:"
	@echo "  all       - Default target, builds release binary"
	@echo "  build     - Build release binary"
	@echo "  bin       - Build binary distribution package"
	@echo "  wasm      - Build WebAssembly version"
	@echo "  doc       - Build documentation"
	@echo "  debug     - Build debug version"
	@echo "  test      - Run tests"
	@echo "  fmt       - Format code with cargo fmt"
	@echo "  clippy    - Run clippy linter"
	@echo "  clean     - Clean build artifacts"
	@echo "  deps      - Show dependency information"
	@echo "  help      - Show this help message"