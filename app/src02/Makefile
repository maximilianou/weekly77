.PHONY: help build test integration-test unit-test clean demo run-inmemory run-file run-file-create fmt check doc

# Variables
BINARY_NAME := src02
DB_FILE := shop_demo.db
DEMO_DB_URL := sqlite:$(DB_FILE)
INMEMORY_DB := sqlite::memory:

help:
	@echo "=========================================="
	@echo "  $(BINARY_NAME) - Rust E-Commerce System"
	@echo "=========================================="
	@echo ""
	@echo "Available targets:"
	@echo ""
	@echo "  Build & Compile:"
	@echo "    make build              - Build the project (debug)"
	@echo "    make build-release      - Build in release mode (optimized)"
	@echo ""
	@echo "  Testing:"
	@echo "    make test               - Run all tests (unit + integration)"
	@echo "    make unit-test          - Run unit tests only"
	@echo "    make integration-test   - Run integration tests only"
	@echo "    make test-verbose       - Run tests with verbose output"
	@echo ""
	@echo "  Running:"
	@echo "    make demo               - Run demo (prints example data)"
	@echo "    make run-inmemory       - Run with in-memory sqlite DB"
	@echo "    make run-file           - Run with file-backed DB ($(DB_FILE))"
	@echo "    make run-file-create    - Init & create DB file, then run"
	@echo ""
	@echo "  Maintenance:"
	@echo "    make clean              - Clean build artifacts and DB file"
	@echo "    make fmt                - Format code with rustfmt"
	@echo "    make check              - Run clippy linter"
	@echo "    make doc                - Generate documentation"
	@echo ""
	@echo "  Examples:"
	@echo "    make build && make test"
	@echo "    make run-file-create"
	@echo "    make demo"
	@echo ""

build:
	@echo "Building $(BINARY_NAME) (debug)..."
	cargo build

build-release:
	@echo "Building $(BINARY_NAME) (release - optimized)..."
	cargo build --release

test: unit-test integration-test
	@echo "All tests passed!"

unit-test:
	@echo "Running unit tests..."
	cargo test --lib

integration-test:
	@echo "Running integration tests..."
	cargo test --tests

test-verbose:
	@echo "Running all tests (verbose)..."
	cargo test --verbose

demo:
	@echo "Running demo with sample data..."
	cargo run --bin $(BINARY_NAME) -- --demo

run-inmemory:
	@echo "Running with in-memory sqlite DB..."
	cargo run --bin $(BINARY_NAME) -- --db-url=$(INMEMORY_DB)

run-file:
	@echo "Running with file-backed DB ($(DB_FILE))..."
	@if [ ! -f "$(DB_FILE)" ]; then \
		echo "Database file not found. Run 'make run-file-create' first."; \
		exit 1; \
	fi
	cargo run --bin $(BINARY_NAME) -- --db-url=$(DEMO_DB_URL)

run-file-create:
	@echo "Initializing DB file ($(DB_FILE))..."
	cargo run --bin $(BINARY_NAME) -- --init-db --db-url=$(DEMO_DB_URL)
	@echo "Database initialized at $(DB_FILE)"

clean:
	@echo "Cleaning project..."
	cargo clean
	@if [ -f "$(DB_FILE)" ]; then \
		rm -f $(DB_FILE); \
		echo "Removed database file: $(DB_FILE)"; \
	fi
	@echo "Clean complete!"

fmt:
	@echo "Formatting code..."
	cargo fmt

check:
	@echo "Running clippy linter..."
	cargo clippy -- -D warnings

doc:
	@echo "Generating documentation..."
	cargo doc --no-deps --open

# Alias for convenience
clean-all: clean
test-all: test
build-all: build-release
