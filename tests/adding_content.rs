//! Adding content safely: a broken Entry is refused by the build with a clear message, and the
//! guide's copy-paste templates are accepted. Each bad Entry in `tests/bad-entries/` is added on
//! its own to the fixture site.

use std::path::Path;

use site_checks::Site;

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// Adds one bad Entry (a path under `tests/bad-entries/`) to the fixture site, with any extra
/// files it needs (like a heavy Clip), and gives back what the build printed when it refused it.
fn refusal_with(entry: &str, extra_files: &[(&str, &[u8])]) -> String {
    let bytes = std::fs::read(format!("{ROOT}/tests/bad-entries/{entry}"))
        .unwrap_or_else(|e| panic!("read the bad Entry {entry}: {e}"));
    let path = format!("content/{entry}");
    let mut files = vec![(path.as_str(), bytes.as_slice())];
    files.extend_from_slice(extra_files);
    match Site::try_build_with(&files) {
        Ok(_) => panic!("the bad Entry {entry} was not refused"),
        Err(message) => message,
    }
}

/// The build must refuse the bad Entry, name its file and state the problem.
fn assert_refused(entry: &str, problem: &str) {
    assert_refused_with(entry, &[], problem);
}

fn assert_refused_with(entry: &str, extra_files: &[(&str, &[u8])], problem: &str) {
    let message = refusal_with(entry, extra_files);
    let expected = format!("Entry content/{entry} refused: {problem}");
    assert!(
        message.contains(&expected),
        "expected {expected:?} in:\n{message}"
    );
    assert!(
        message.contains("See docs/adding-content.md."),
        "the refusal points at the guide:\n{message}"
    );
}

#[test]
fn an_unknown_format_is_refused() {
    assert_refused(
        "videos/unknown-format.md",
        "unknown Format \"Podcasts\": `format` must be one of Real or BS, GitHub tools, Storytelling, Talking head, Random builds, Make it better, Vlogs, Funny videos, Lifestyle carousel, Educational carousel, With Gena, Other.",
    );
}

#[test]
fn a_video_with_no_format_is_refused() {
    assert_refused("videos/no-format.md", "the Video has no Format:");
}

#[test]
fn a_verdict_that_is_not_real_or_bs_is_refused() {
    assert_refused(
        "videos/verdict-not-real-or-bs.md",
        "the Verdict \"maybe\" is not \"real\" or \"bs\".",
    );
}

#[test]
fn a_verdict_on_a_video_that_is_not_real_or_bs_is_refused() {
    assert_refused(
        "videos/verdict-on-other-format.md",
        "`verdict` is only for Real or BS Videos, and the Format of this Video is \"Storytelling\".",
    );
}

#[test]
fn an_episode_on_a_video_that_is_not_real_or_bs_is_refused() {
    assert_refused(
        "videos/episode-on-other-format.md",
        "`episode` is only for Real or BS Videos, and the Format of this Video is \"GitHub tools\".",
    );
}

#[test]
fn a_tested_line_on_a_video_that_is_not_real_or_bs_is_refused() {
    assert_refused(
        "videos/tested-on-other-format.md",
        "`tested` is only for Real or BS Videos, and the Format of this Video is \"Talking head\".",
    );
}

#[test]
fn a_real_or_bs_video_with_no_verdict_is_refused() {
    assert_refused(
        "videos/real-or-bs-no-verdict.md",
        "a Real or BS Video needs a Verdict:",
    );
}

#[test]
fn a_real_or_bs_video_with_no_episode_is_refused() {
    for entry in [
        "videos/real-or-bs-no-episode.md",
        "videos/real-or-bs-episode-not-a-number.md",
    ] {
        assert_refused(entry, "a Real or BS Video needs `episode`:");
    }
}

#[test]
fn a_real_or_bs_video_with_no_tested_line_is_refused() {
    assert_refused(
        "videos/real-or-bs-no-tested.md",
        "a Real or BS Video needs `tested`:",
    );
}

#[test]
fn a_video_with_no_cover_is_refused() {
    assert_refused("videos/no-cover.md", "the Video has no cover:");
}

#[test]
fn a_cover_file_that_does_not_exist_is_refused() {
    for entry in [
        "videos/cover-file-missing.md",
        "playbooks/cover-file-missing.md",
    ] {
        assert_refused(
            entry,
            "the cover file static/covers/not-there.png does not exist.",
        );
    }
}

#[test]
fn a_cover_that_is_not_a_png_or_jpeg_is_refused() {
    assert_refused(
        "videos/cover-not-png-or-jpeg.md",
        "`cover` must be the path of a PNG or JPEG under static/",
    );
}

#[test]
fn an_entry_with_no_title_is_refused() {
    for entry in ["videos/no-title.md", "playbooks/no-title.md"] {
        assert_refused(entry, "the Entry has no title:");
    }
}

#[test]
fn an_entry_with_no_date_is_refused() {
    for entry in ["videos/no-date.md", "playbooks/no-date.md"] {
        assert_refused(entry, "the Entry has no date:");
    }
}

/// Zola leaves an Entry with no date out of the site with only a warning, so `zola build` alone
/// cannot refuse it. This check reads the real Entries in `content/`, and `just check` runs it
/// before the build.
#[test]
fn the_real_entries_all_have_a_date() {
    let refused = site_checks::entries_without_a_date(Path::new(&format!("{ROOT}/content")));
    assert!(refused.is_empty(), "\n{}\n", refused.join("\n"));
}

#[test]
fn a_video_with_no_clip_is_refused() {
    assert_refused("videos/no-clip.md", "the Video has no Clip:");
}

#[test]
fn a_clip_that_is_not_an_mp4_in_the_clips_folder_is_refused() {
    assert_refused(
        "videos/clip-not-mp4.md",
        "`clip` must be the path of an MP4 in static/clips/",
    );
}

#[test]
fn a_clip_file_that_does_not_exist_is_refused() {
    assert_refused(
        "videos/clip-file-missing.md",
        "the Clip file static/clips/not-there.mp4 does not exist: make it with the Clip command.",
    );
}

#[test]
fn a_clip_over_the_budget_is_refused() {
    let heavy = vec![0u8; 400_001];
    assert_refused_with(
        "videos/clip-over-budget.md",
        &[("static/clips/heavy.mp4", &heavy)],
        "the Clip static/clips/heavy.mp4 is 400001 bytes, over the budget of 400000 bytes (400 KB):",
    );
}

#[test]
fn a_clip_at_the_budget_is_accepted() {
    let bytes = std::fs::read(format!(
        "{ROOT}/tests/bad-entries/videos/clip-over-budget.md"
    ))
    .unwrap();
    let at_budget = vec![0u8; 400_000];
    Site::try_build_with(&[
        ("content/videos/clip-at-budget.md", &bytes),
        ("static/clips/heavy.mp4", &at_budget),
    ])
    .unwrap_or_else(|message| panic!("a Clip of exactly 400 KB was refused:\n{message}"));
}

#[test]
fn a_video_with_no_still_is_refused() {
    assert_refused("videos/no-still.md", "the Video has no still:");
}

#[test]
fn a_still_file_that_does_not_exist_is_refused() {
    assert_refused(
        "videos/still-file-missing.md",
        "the still file static/clips/not-there.webp does not exist.",
    );
}

/// Zola can neither see an MP4 nor read its size, so this check reads the real Video Entries
/// and their Clips; `just check` runs it before the build.
#[test]
fn the_real_videos_all_have_a_clip_under_budget() {
    let refused = site_checks::clips_refused(Path::new(ROOT));
    assert!(refused.is_empty(), "\n{}\n", refused.join("\n"));
}

#[test]
fn a_playbook_reference_that_matches_no_playbook_is_refused() {
    assert_refused(
        "videos/playbook-not-found.md",
        "`playbook` is \"playbooks/not-there.md\", which matches no Playbook Entry.",
    );
}

#[test]
fn a_link_that_is_not_a_well_formed_https_address_is_refused() {
    for (entry, problem) in [
        (
            "videos/tiktok-not-https.md",
            "`tiktok` is not a well-formed https address: \"http://www.tiktok.com/@fixture/video/1\".",
        ),
        (
            "videos/instagram-malformed.md",
            "`instagram` is not a well-formed https address: \"https://www.instagram com/reel/FIXTURE/\".",
        ),
        (
            "playbooks/pdf-not-https.md",
            "`pdf` is not a well-formed https address: \"example.com/playbooks/bad.pdf\".",
        ),
    ] {
        assert_refused(entry, problem);
    }
}

#[test]
fn a_playbook_with_no_page_and_no_pdf_is_refused() {
    assert_refused(
        "playbooks/no-page-no-pdf.md",
        "the Playbook has no page and no PDF:",
    );
}

#[test]
fn every_bad_entry_has_a_check() {
    let checks = std::fs::read_to_string(format!("{ROOT}/tests/adding_content.rs")).unwrap();
    for folder in ["videos", "playbooks"] {
        for file in std::fs::read_dir(format!("{ROOT}/tests/bad-entries/{folder}")).unwrap() {
            let name = file.unwrap().file_name().into_string().unwrap();
            assert!(
                checks.contains(&format!("\"{folder}/{name}\"")),
                "tests/bad-entries/{folder}/{name} is not used by a check"
            );
        }
    }
}

/// Zola renders pages in no fixed order. A page that reads the Entries (through `get_section`,
/// `get_page` or `section.pages`) can meet a broken Entry before `templates/entry.html` refuses it
/// and stop the build with an error of its own, so it must refuse the broken Entries first.
#[test]
fn every_page_that_reads_the_entries_refuses_the_broken_ones_first() {
    let mut reading = 0;
    for file in std::fs::read_dir(format!("{ROOT}/templates")).unwrap() {
        let path = file.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        if !name.ends_with(".html") || name == "entry.html" {
            continue;
        }
        let template = without_comments(&std::fs::read_to_string(&path).unwrap());
        let reads = ["get_section(", "get_page(", "section.pages", "paginator"]
            .iter()
            .filter_map(|read| template.find(read))
            .min();
        let Some(first_read) = reads else { continue };
        reading += 1;
        let refuses = template.find("{{<validate.entries formats={config.extra.formats} />}}");
        assert!(
            refuses.is_some_and(|at| at < first_read),
            "templates/{name} reads the Entries: put \
             `{{{{<validate.entries formats={{config.extra.formats}} />}}}}` before its first read"
        );
    }
    assert!(
        reading >= 2,
        "the Real or BS and Playbooks pages read the Entries"
    );
}

/// A template without its `{# ... #}` comments.
fn without_comments(template: &str) -> String {
    let mut kept = String::new();
    let mut rest = template;
    while let Some(start) = rest.find("{#") {
        kept.push_str(&rest[..start]);
        rest = rest[start..]
            .find("#}")
            .map_or("", |end| &rest[start + end + 2..]);
    }
    kept + rest
}

/// The four copy-paste templates of the guide, in order: a Video with no Verdict, a Real or BS
/// Video with a Verdict and a Playbook, that Playbook (PDF only), and a Playbook page.
fn guide_templates() -> Vec<String> {
    let guide = std::fs::read_to_string(format!("{ROOT}/docs/adding-content.md")).unwrap();
    guide
        .split("```toml\n")
        .skip(1)
        .map(|rest| rest.split("```").next().unwrap().to_owned())
        .collect()
}

#[test]
fn the_templates_of_the_guide_are_accepted() {
    let templates = guide_templates();
    assert_eq!(templates.len(), 4, "the guide holds four templates");
    let cover = std::fs::read(format!("{ROOT}/tests/fixtures/covers/fixture-cover.png")).unwrap();
    let clip = std::fs::read(format!("{ROOT}/tests/fixtures/clips/fixture-clip.mp4")).unwrap();
    let still = std::fs::read(format!("{ROOT}/tests/fixtures/clips/fixture-clip.webp")).unwrap();
    let site = Site::try_build_with(&[
        (
            "content/videos/github-tools-repo-radar.md",
            templates[0].as_bytes(),
        ),
        (
            "content/videos/real-or-bs-02-phone-use.md",
            templates[1].as_bytes(),
        ),
        ("content/playbooks/phone-use.md", templates[2].as_bytes()),
        ("content/playbooks/voice-notes.md", templates[3].as_bytes()),
        ("static/covers/github-tools-repo-radar.png", &cover),
        ("static/covers/real-or-bs-02-phone-use.png", &cover),
        ("static/covers/phone-use-playbook.png", &cover),
        ("static/covers/voice-notes-playbook.png", &cover),
        ("static/clips/github-tools-repo-radar.mp4", &clip),
        ("static/clips/github-tools-repo-radar.webp", &still),
        ("static/clips/real-or-bs-02-phone-use.mp4", &clip),
        ("static/clips/real-or-bs-02-phone-use.webp", &still),
    ])
    .unwrap_or_else(|message| panic!("the guide's templates were refused:\n{message}"));
    for entry in [
        "videos/github-tools-repo-radar/index.html",
        "videos/real-or-bs-02-phone-use/index.html",
        "playbooks/phone-use/index.html",
        "playbooks/voice-notes/index.html",
    ] {
        assert!(site.has_file(entry), "{entry} was built");
    }
    let page = site_checks::Page::from_file(&site, "playbooks/voice-notes/index.html");
    assert_eq!(
        page.count(".pb-prompt"),
        1,
        "the Playbook page template is a page"
    );
}

/// The guide explains every refusal: each message a session can meet is quoted in it.
#[test]
fn the_guide_explains_every_refusal() {
    let guide = std::fs::read_to_string(format!("{ROOT}/docs/adding-content.md")).unwrap();
    let checks =
        std::fs::read_to_string(format!("{ROOT}/templates/components/validate.html")).unwrap();
    for phrase in [
        "unknown Format",
        "has no Format",
        "is not \"real\" or \"bs\"",
        "is only for Real or BS Videos",
        "needs a Verdict",
        "needs `episode`",
        "needs `tested`",
        "has no cover",
        "does not exist",
        "must be the path of a PNG or JPEG",
        "has no title",
        "has no date",
        "matches no Playbook Entry",
        "is not a well-formed https address",
        "has no page and no PDF",
        "has no Clip",
        "must be the path of an MP4",
        "has no still",
        "must be the path of a WebP",
    ] {
        assert!(checks.contains(phrase), "{phrase:?} is a refusal message");
        assert!(guide.contains(phrase), "the guide explains {phrase:?}");
    }
    let harness = std::fs::read_to_string(format!("{ROOT}/src/lib.rs")).unwrap();
    for phrase in ["the Clip file", "over the budget of"] {
        assert!(harness.contains(phrase), "{phrase:?} is a refusal message");
        assert!(guide.contains(phrase), "the guide explains {phrase:?}");
    }
}
