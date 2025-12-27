.PHONY: build test coverage clean fmt lint check all

# Default target
all: check build test

# Build the project
build:
	cargo build --release

# Run tests
test:
	cargo test --all-targets

# Run tests with coverage using tarpaulin
coverage:
	cargo tarpaulin --out Html --out Lcov --output-dir coverage --ignore-tests

# Clean build artifacts
clean:
	cargo clean
	rm -rf coverage

# Format code
fmt:
	cargo fmt --all

# Check formatting
fmt-check:
	cargo fmt --all -- --check

# Run clippy lints
lint:
	cargo clippy --all-targets -- -D warnings

# Run all checks (format, lint, test)
check: fmt-check lint

# Install development dependencies
dev-deps:
	cargo install cargo-tarpaulin

# Help target
help:
	@echo "Available targets:"
	@echo "  all       - Run check, build, and test"
	@echo "  build     - Build release binary"
	@echo "  test      - Run all tests"
	@echo "  coverage  - Generate code coverage report"
	@echo "  clean     - Remove build artifacts and coverage"
	@echo "  fmt       - Format code"
	@echo "  fmt-check - Check code formatting"
	@echo "  lint      - Run clippy lints"
	@echo "  check     - Run fmt-check and lint"
	@echo "  dev-deps  - Install development dependencies"
