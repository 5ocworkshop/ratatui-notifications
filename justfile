# FILE: justfile - Task runner for ratatui-notifications
# VERSION: 1.1.0
# WCTX: Adding code generation feature
# CLOG: Added cookbook example recipes

# Default recipe: show available commands
default:
    @just --list

# Run the interactive demo
demo:
    cargo run --example demo

# Run the cookbook (curated recipes with code snippets)
cookbook:
    cargo run --example cookbook

# Run all tests
test:
    cargo test

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# Run a specific test
test-one TEST:
    cargo test {{TEST}} -- --nocapture

# Check compilation without building
check:
    cargo check

# Build the library
build:
    cargo build

# Build in release mode
build-release:
    cargo build --release

# Run clippy lints
lint:
    cargo clippy -- -D warnings

# Format code
fmt:
    cargo fmt

# Check formatting without modifying
fmt-check:
    cargo fmt -- --check

# Run all quality checks (format, lint, test)
ci: fmt-check lint test

# Generate documentation
doc:
    cargo doc --no-deps

# Open documentation in browser
doc-open:
    cargo doc --no-deps --open

# Clean build artifacts
clean:
    cargo clean

# Watch for changes and run tests
watch:
    cargo watch -x test

# Watch for changes and run demo
watch-demo:
    cargo watch -x 'run --example demo'

# Watch for changes and run cookbook
watch-cookbook:
    cargo watch -x 'run --example cookbook'

# Show dependency tree
deps:
    cargo tree

# Update dependencies
update:
    cargo update

# FILE: justfile - Task runner for ratatui-notifications
# END OF VERSION: 1.1.0
