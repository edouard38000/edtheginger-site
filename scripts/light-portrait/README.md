# light-portrait

Makes the Poster's lit portrait, `static/home/portrait-lit.webp`, and its grain tile,
`static/home/grain.png`, from `brand-assets/portrait/portrait-cutout.png`. Run it by hand whenever
the cut-out or the beam in `content/_index.md` (`[extra.beam]`) changes, then commit the outputs:

```sh
cargo run --release --manifest-path scripts/light-portrait/Cargo.toml
```

The beam's numbers live only in `content/_index.md`: this script lights the strip between the two
lines with them, and the home template draws the two white diagonal lines from the same numbers.
`beam-used.toml` records what the published image was made with; `tests/home.rs` fails if it no
longer matches, which means the script needs running again.

It is Rust, like the rest of the repo's tooling, and is its own small workspace so the site checks
never build it.
