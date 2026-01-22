# Run formatting check
fmt-check:
    cargo fmt --all -- --check

# Run clippy with all warnings as errors
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Run tests
test:
    cargo test

# Run all checks (formatting, clippy, tests)
check: fmt-check clippy test

# Format code
fmt:
    cargo fmt --all

# Build release binary
build:
    cargo build --release

# Install git hooks
install-hooks:
    #!/usr/bin/env bash
    echo "Installing git hooks..."

    # Pre-commit hook
    cat > .git/hooks/pre-commit << 'EOF'
    #!/bin/sh

    if command -v just &> /dev/null; then
        just fmt-check
    else
        cargo fmt --all -- --check
    fi
    EOF
    chmod +x .git/hooks/pre-commit

    # Pre-push hook
    cat > .git/hooks/pre-push << 'EOF'
    #!/bin/sh

    if command -v just &> /dev/null; then
        just clippy
    else
        cargo clippy --all-targets --all-features -- -D warnings
    fi
    EOF
    chmod +x .git/hooks/pre-push

    echo "Git hooks installed successfully!"
    echo "- pre-commit: runs formatting check"
    echo "- pre-push: runs clippy lints"
