//! Makes a Video's Clip and its still from a source video file.
//!
//! A Clip is the first five seconds of the Video (its hook): MP4, H.264, 360 x 640, 60 fps (Chrome
//! draws the page at the frame rate of the videos on it, so the Wheel turns at 60 fps), no audio
//! track, fast-start (the index at the front, so a phone starts playing it at once). Its still is
//! the Clip's first frame as WebP. Both are made with `ffmpeg`, which must be on the PATH (or
//! named by `FFMPEG`); `ffprobe` (or `FFPROBE`) reads them back.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// How long a Clip is, in seconds.
pub const SECONDS: u32 = 5;
/// The Clip's frame rate: Chrome draws the whole page at the rate of the videos playing on it.
pub const FPS: u32 = 60;
/// The Clip's size in pixels: portrait 9:16.
pub const WIDTH: u32 = 360;
pub const HEIGHT: u32 = 640;
/// The site refuses a Clip over this many bytes (`src/lib.rs`, `CLIP_BUDGET_BYTES`).
pub const BUDGET_BYTES: u64 = 400_000;
/// The command aims well under the budget: it raises the compression until the Clip fits this.
pub const TARGET_BYTES: u64 = 300_000;
/// The x264 quality steps tried in turn (higher is smaller).
const CRF_STEPS: [u32; 5] = [27, 30, 33, 36, 40];
/// WebP quality of the still.
const STILL_QUALITY: f32 = 80.0;

/// What [`make_clip`] wrote.
#[derive(Debug)]
pub struct Made {
    pub clip: PathBuf,
    pub still: PathBuf,
    pub clip_bytes: u64,
    pub still_bytes: u64,
}

/// Makes `<out_dir>/<name>.mp4` and `<out_dir>/<name>.webp` from `source`.
pub fn make_clip(source: &Path, out_dir: &Path, name: &str) -> Result<Made, String> {
    if !source.is_file() {
        return Err(format!("no source video at {}", source.display()));
    }
    if name.is_empty()
        || !name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return Err(format!(
            "the name {name:?} must be the Entry's file name without `.md`: lowercase letters, digits and hyphens"
        ));
    }
    fs::create_dir_all(out_dir).map_err(|e| format!("create {}: {e}", out_dir.display()))?;
    let clip = out_dir.join(format!("{name}.mp4"));
    let still = out_dir.join(format!("{name}.webp"));

    let mut clip_bytes = u64::MAX;
    for crf in CRF_STEPS {
        encode(source, &clip, crf)?;
        clip_bytes = file_size(&clip)?;
        if clip_bytes <= TARGET_BYTES {
            break;
        }
    }
    if clip_bytes > BUDGET_BYTES {
        return Err(format!(
            "the Clip {} is {} KB even at the lowest quality, over the {} KB budget",
            clip.display(),
            clip_bytes / 1000,
            BUDGET_BYTES / 1000
        ));
    }

    write_still(&clip, &still)?;
    Ok(Made {
        still_bytes: file_size(&still)?,
        clip,
        still,
        clip_bytes,
    })
}

fn encode(source: &Path, clip: &Path, crf: u32) -> Result<(), String> {
    let filter = format!(
        "scale={WIDTH}:{HEIGHT}:force_original_aspect_ratio=increase,crop={WIDTH}:{HEIGHT},fps={FPS},format=yuv420p"
    );
    let crf = crf.to_string();
    let seconds = SECONDS.to_string();
    run(
        &ffmpeg(),
        &[
            "-v",
            "error",
            "-y",
            "-i",
            &path_arg(source)?,
            "-t",
            &seconds,
            "-map",
            "0:v:0",
            "-vf",
            &filter,
            "-c:v",
            "libx264",
            "-preset",
            "slow",
            "-profile:v",
            "main",
            "-crf",
            &crf,
            "-maxrate",
            "600k",
            "-bufsize",
            "1200k",
            "-an",
            "-sn",
            "-dn",
            "-map_metadata",
            "-1",
            "-movflags",
            "+faststart",
            &path_arg(clip)?,
        ],
    )
    .map(drop)
}

/// The Clip's first frame, as WebP.
fn write_still(clip: &Path, still: &Path) -> Result<(), String> {
    let png = run(
        &ffmpeg(),
        &[
            "-v",
            "error",
            "-i",
            &path_arg(clip)?,
            "-frames:v",
            "1",
            "-f",
            "image2pipe",
            "-c:v",
            "png",
            "-",
        ],
    )?;
    let frame = image::load_from_memory_with_format(&png, image::ImageFormat::Png)
        .map_err(|e| format!("read the first frame of {}: {e}", clip.display()))?
        .to_rgb8();
    let webp = webp::Encoder::from_rgb(frame.as_raw(), frame.width(), frame.height())
        .encode(STILL_QUALITY);
    fs::write(still, &*webp).map_err(|e| format!("write {}: {e}", still.display()))
}

/// What `ffprobe` reads in a media file.
#[derive(Debug)]
pub struct Probe {
    /// Seconds, from the container.
    pub duration: f64,
    pub width: u32,
    pub height: u32,
    /// Frames per second of the first video stream (0 when there is none).
    pub fps: f64,
    /// The codec of each stream, like `h264` or `aac`.
    pub codecs: Vec<String>,
    /// The type of each stream, like `video` or `audio`.
    pub kinds: Vec<String>,
}

/// Reads the duration, the size of the first video stream, and every stream of a file.
pub fn probe(file: &Path) -> Result<Probe, String> {
    let out = run(
        &ffprobe(),
        &[
            "-v",
            "error",
            "-show_entries",
            "format=duration:stream=codec_type,codec_name,width,height,avg_frame_rate",
            "-of",
            "default=noprint_wrappers=1",
            &path_arg(file)?,
        ],
    )?;
    let text = String::from_utf8_lossy(&out);
    let mut probe = Probe {
        duration: 0.0,
        width: 0,
        height: 0,
        fps: 0.0,
        codecs: Vec::new(),
        kinds: Vec::new(),
    };
    for line in text.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        match key {
            "duration" => probe.duration = value.parse().unwrap_or(0.0),
            "codec_name" => probe.codecs.push(value.to_owned()),
            "codec_type" => probe.kinds.push(value.to_owned()),
            "avg_frame_rate" if probe.fps == 0.0 => probe.fps = rate(value),
            "width" if probe.width == 0 => probe.width = value.parse().unwrap_or(0),
            "height" if probe.height == 0 => probe.height = value.parse().unwrap_or(0),
            _ => {}
        }
    }
    Ok(probe)
}

/// A rate like `60/1` as a number; 0 when it cannot be read.
fn rate(value: &str) -> f64 {
    let (num, den) = value.split_once('/').unwrap_or((value, "1"));
    match (num.parse::<f64>(), den.parse::<f64>()) {
        (Ok(num), Ok(den)) if den > 0.0 => num / den,
        _ => 0.0,
    }
}

/// Whether an MP4 is fast-start: its index (`moov`) comes before its media data (`mdat`).
pub fn is_fast_start(mp4: &[u8]) -> bool {
    let find = |tag: &[u8]| mp4.windows(4).position(|window| window == tag);
    matches!((find(b"moov"), find(b"mdat")), (Some(moov), Some(mdat)) if moov < mdat)
}

fn ffmpeg() -> String {
    std::env::var("FFMPEG").unwrap_or_else(|_| "ffmpeg".to_owned())
}

fn ffprobe() -> String {
    std::env::var("FFPROBE").unwrap_or_else(|_| "ffprobe".to_owned())
}

fn path_arg(path: &Path) -> Result<String, String> {
    path.to_str()
        .map(str::to_owned)
        .ok_or_else(|| format!("the path {} is not valid UTF-8", path.display()))
}

fn file_size(path: &Path) -> Result<u64, String> {
    fs::metadata(path)
        .map(|m| m.len())
        .map_err(|e| format!("read {}: {e}", path.display()))
}

/// Runs a tool and gives back its standard output.
fn run(program: &str, args: &[&str]) -> Result<Vec<u8>, String> {
    let output = Command::new(program).args(args).output().map_err(|e| {
        format!("could not run `{program}` (is it installed and on the PATH?): {e}")
    })?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(format!(
            "`{program}` failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
