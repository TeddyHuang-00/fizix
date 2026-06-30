# Format code
format:
    cargo +nightly fmt --all
    cargo sort
    cargo sort-derives

# Check unused dependencies (nightly only, run separately if needed)
deps:
    cargo +nightly udeps

# Check for errors
check: && format
    cargo clippy --fix --all-targets --all-features

# Unit tests
test: check
    cargo test

# Coverage report
coverage: check
    cargo tarpaulin --out Html --output-dir coverage
