//! Makes a Clip from a short fixture video (generated here with ffmpeg: 8 seconds, landscape,
//! with a sound track) and checks what the site needs from it.

use std::path::Path;
use std::process::Command;

fn fixture_video(dir: &Path) -> std::path::PathBuf {
    let source = dir.join("source.mp4");
    let ffmpeg = std::env::var("FFMPEG").unwrap_or_else(|_| "ffmpeg".to_owned());
    let status = Command::new(&ffmpeg)
        .args([
            "-v",
            "error",
            "-y",
            "-f",
            "lavfi",
            "-i",
            "testsrc2=size=1280x720:rate=30:duration=8",
            "-f",
            "lavfi",
            "-i",
            "sine=frequency=440:duration=8",
            "-c:v",
            "libx264",
            "-c:a",
            "aac",
            "-shortest",
        ])
        .arg(&source)
        .status()
        .unwrap_or_else(|e| panic!("run `{ffmpeg}` (is ffmpeg installed?): {e}"));
    assert!(status.success(), "ffmpeg made the fixture video");
    source
}

#[test]
fn a_clip_is_five_silent_seconds_at_360_by_640_under_budget_with_a_webp_still() {
    let dir = tempfile::tempdir().unwrap();
    let source = fixture_video(dir.path());
    let source_probe = clip::probe(&source).unwrap();
    assert!(
        source_probe.kinds.contains(&"audio".to_owned()),
        "the fixture has sound, so the check on the Clip means something"
    );

    let out = dir.path().join("clips");
    let made = clip::make_clip(&source, &out, "fixture-video").unwrap();
    assert_eq!(made.clip, out.join("fixture-video.mp4"));
    assert_eq!(made.still, out.join("fixture-video.webp"));

    let probe = clip::probe(&made.clip).unwrap();
    assert!(
        (4.9..=5.1).contains(&probe.duration),
        "the Clip lasts five seconds, not {}",
        probe.duration
    );
    assert_eq!((probe.width, probe.height), (clip::WIDTH, clip::HEIGHT));
    assert_eq!(
        probe.kinds,
        ["video"],
        "one video stream and no audio track"
    );
    assert_eq!(probe.codecs, ["h264"]);

    let bytes = std::fs::read(&made.clip).unwrap();
    assert_eq!(bytes.len() as u64, made.clip_bytes);
    assert!(
        made.clip_bytes <= clip::TARGET_BYTES,
        "the Clip is {} bytes, over the target (well under the budget)",
        made.clip_bytes
    );
    assert!(clip::is_fast_start(&bytes), "the Clip is fast-start");

    let still = std::fs::read(&made.still).unwrap();
    assert!(
        still.starts_with(b"RIFF") && &still[8..12] == b"WEBP",
        "the still is WebP"
    );
    let still_probe = clip::probe(&made.still).unwrap();
    assert_eq!(
        (still_probe.width, still_probe.height),
        (clip::WIDTH, clip::HEIGHT)
    );
}

#[test]
fn a_bad_entry_name_or_a_missing_source_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let missing = clip::make_clip(&dir.path().join("none.mp4"), dir.path(), "x").unwrap_err();
    assert!(missing.contains("no source video"), "{missing}");

    let source = dir.path().join("source.mp4");
    std::fs::write(&source, b"").unwrap();
    let bad = clip::make_clip(&source, dir.path(), "Phone Use").unwrap_err();
    assert!(bad.contains("lowercase letters"), "{bad}");
}
