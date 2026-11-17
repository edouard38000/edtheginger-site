# One command builds the site and runs every check: `just check`.

# Formatting, lints, the test suite (builds the site from fixture Entries), then the real site.
check: fmt-check clippy test build

# Build the real site into public/ (also checks internal links).
build:
    zola check --skip-external-links
    zola build

# Serve the real site locally with live reload.
serve:
    zola serve

test:
    cargo test

clippy:
    cargo clippy --all-targets -- -D warnings

fmt-check:
    cargo fmt --check

fmt:
    cargo fmt
