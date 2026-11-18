//! `clip <source video> <entry name>`: writes `static/clips/<entry name>.mp4` and
//! `static/clips/<entry name>.webp` in this repo. See `docs/adding-content.md`, "Add the Clip".

use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [source, name] = args.as_slice() else {
        eprintln!(
            "usage: cargo run --release --manifest-path scripts/clip/Cargo.toml -- <source video> <entry name>\n\
             The entry name is the Entry's file name without `.md`, like real-or-bs-02-phone-use."
        );
        return ExitCode::FAILURE;
    };
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out_dir: PathBuf = repo.join("static").join("clips");
    match clip::make_clip(Path::new(source), &out_dir, name) {
        Ok(made) => {
            println!(
                "Wrote static/clips/{name}.mp4 ({} KB) and static/clips/{name}.webp ({} KB).\n\
                 In the Entry, under [extra]:\n\
                 clip = \"clips/{name}.mp4\"\n\
                 still = \"clips/{name}.webp\"",
                made.clip_bytes / 1000,
                made.still_bytes / 1000,
            );
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("clip: {message}");
            ExitCode::FAILURE
        }
    }
}
