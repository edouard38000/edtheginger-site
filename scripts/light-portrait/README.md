# light-portrait

**Not in use right now.** The Poster shows the AI relight of Edouard's photo
(`static/home/portrait-relit-ai.webp`, from `brand-assets/portrait/portrait-relit-ai.png`), which
carries its own light; the page adds none. Keep this script for a reshoot: to use it again, measure
the beam on the cut-out, set `[extra.beam]`, run the script, and set `portrait` in
`content/_index.md` to `home/portrait-lit.webp`. `tests/home.rs` then checks the image was made with
the current beam.

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
