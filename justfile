# One command builds the site and runs every check: `just check`.

# Formatting, lints, the test suite (builds the site from fixture Entries), then the real site.
check: fmt-check clippy test clip-test build

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

# The Clip command (scripts/clip, its own crate): formatting, lints and its test, which makes a
# Clip from a generated video. Needs ffmpeg and ffprobe.
clip-test:
    cargo fmt --check --manifest-path scripts/clip/Cargo.toml
    cargo clippy --manifest-path scripts/clip/Cargo.toml --all-targets -- -D warnings
    cargo test --manifest-path scripts/clip/Cargo.toml
