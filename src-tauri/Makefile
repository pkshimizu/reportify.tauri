# Makefile for Rust backend development commands

.PHONY: lint format check-format check clean test build migrate-up migrate-down migrate-fresh migrate-generate

# Run clippy linter with warnings as errors
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Format all Rust code
format:
	cargo fmt --all

# Check if code is properly formatted
check-format:
	cargo fmt --all -- --check

# Compile check without building
check:
	cargo check --all-targets --all-features

# Clean build artifacts
clean:
	cargo clean

# Run tests
test:
	cargo test

# Build the project
build:
	cargo build

# Run all quality checks (format, lint, test)
ci: check-format lint test

# Development helper: format and run quick checks
dev: format check

# Database migration commands
# Apply all pending migrations
migrate-up:
	cd migration && cargo run -- up

# Rollback the last applied migration
migrate-down:
	cd migration && cargo run -- down

# Drop all tables and re-run all migrations
migrate-fresh:
	cd migration && cargo run -- fresh

# Generate a new migration file
migrate-generate:
	@read -p "Enter migration name: " name; \
	sea-orm-cli migrate generate -d migration $$name