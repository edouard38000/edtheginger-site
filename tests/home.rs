//! The home page: the Poster (lit portrait, beam lines, tool tiles, captions, title), the two entry
//! points and the latest Video.
//! Built from the fixture Entries, where the newest Video is "Voice Clone" (2026-01-19, Real or BS,
//! Instagram only), the Verdicts are two `real` and one `bs`, and there are two Playbooks.

use std::sync::OnceLock;

use site_checks::{Page, Site, SUB_PATH_BASE_URL};

fn site() -> &'static Site {
    static SITE: OnceLock<Site> = OnceLock::new();
    SITE.get_or_init(Site::build)
}

fn home() -> Page {
    site().page("")
}

/// The path of a file of the built site, from its address.
fn built_file(address: &str) -> &str {
    address
        .strip_prefix(&format!("{SUB_PATH_BASE_URL}/"))
        .unwrap_or_else(|| panic!("{address} is not served by the site itself"))
}

#[test]
fn the_poster_title_is_the_one_h1() {
    let home = home();
    assert_eq!(home.count("h1"), 1);
    let title = home.texts(".poster h1").join(" ");
    let words: Vec<&str> = title.split_whitespace().collect();
    assert_eq!(words.join(" ").to_uppercase(), "EDOUARD THE GINGER");
}

#[test]
fn the_poster_shows_the_lit_portrait_resized_with_alt_text() {
    let home = home();
    let portraits = home.attributes(".poster img.poster__portrait[alt]:not([alt=''])", "src");
    assert_eq!(portraits.len(), 1, "one portrait on the Poster");
    let file = built_file(&portraits[0]);
    assert!(
        file.starts_with("processed_images/"),
        "the portrait is resized at build time, not the original: {file}"
    );
    assert!(site().has_file(file), "{file} is in the built site");
    let size = std::fs::metadata(site().output().join(file)).unwrap().len();
    assert!(size < 150_000, "the portrait served first is {size} bytes");
    assert_eq!(
        home.count(".poster__photo, .poster__frame, .poster__tape"),
        0,
        "the taped photo frame is gone from the Poster"
    );
}

/// The number values of one table of the home page's front matter, e.g. `[extra.beam]`.
fn front_matter_numbers(table: &str) -> Vec<(String, f64)> {
    let root = env!("CARGO_MANIFEST_DIR");
    let text = std::fs::read_to_string(format!("{root}/content/_index.md")).unwrap();
    numbers_after(&text, &format!("[{table}]"))
}

/// `key = number` lines after `header` (or from the start when it is empty), up to the next table.
fn numbers_after(text: &str, header: &str) -> Vec<(String, f64)> {
    let mut lines = text.lines();
    if !header.is_empty() {
        lines
            .by_ref()
            .find(|line| line.trim() == header)
            .unwrap_or_else(|| panic!("{header} is in the file"));
    }
    lines
        .map(|line| line.split('#').next().unwrap().trim())
        .take_while(|line| !line.starts_with('['))
        .filter_map(|line| line.split_once('='))
        .filter_map(|(key, value)| Some((key.trim().to_owned(), value.trim().parse().ok()?)))
        .collect()
}

fn number(values: &[(String, f64)], key: &str) -> f64 {
    values
        .iter()
        .find(|(k, _)| k == key)
        .unwrap_or_else(|| panic!("{key} is set"))
        .1
}

/// The width and height of a WebP file with an alpha channel (a VP8X canvas).
fn webp_size(bytes: &[u8]) -> (f64, f64) {
    assert_eq!(&bytes[0..4], b"RIFF");
    assert_eq!(&bytes[8..12], b"WEBP");
    assert_eq!(&bytes[12..16], b"VP8X", "an extended WebP, with alpha");
    let u24 = |at: usize| {
        u32::from(bytes[at]) | u32::from(bytes[at + 1]) << 8 | u32::from(bytes[at + 2]) << 16
    };
    (f64::from(u24(24) + 1), f64::from(u24(27) + 1))
}

fn attribute_numbers(home: &Page, selector: &str, attribute: &str) -> Vec<f64> {
    home.attributes(selector, attribute)
        .iter()
        .map(|value| {
            value
                .parse()
                .unwrap_or_else(|_| panic!("{value} is a number"))
        })
        .collect()
}

fn assert_close(found: &[f64], expected: &[f64], what: &str) {
    assert_eq!(found.len(), expected.len(), "{what}");
    for (f, e) in found.iter().zip(expected) {
        assert!(
            (f - e).abs() < 0.01,
            "{what}: {found:?} is not {expected:?}"
        );
    }
}

/// The lines are the edges of the light: the template draws them from `[extra.beam]`, in the
/// pixels of the published portrait, so they sit on the lit strip at every width.
#[test]
fn the_beam_lines_are_drawn_from_the_beam_that_lit_the_portrait() {
    let root = env!("CARGO_MANIFEST_DIR");
    let beam = front_matter_numbers("extra.beam");
    let (x, y, slope, width) = (
        number(&beam, "x"),
        number(&beam, "y"),
        number(&beam, "slope"),
        number(&beam, "width"),
    );
    let image = std::fs::read(format!("{root}/static/home/portrait-lit.webp")).unwrap();
    let (w, h) = webp_size(&image);

    let home = home();
    assert_eq!(
        home.attributes(".poster__lines", "viewBox"),
        [format!("{} 0 {} {}", -2.0 * w, 5.0 * w, h)],
        "the lines are drawn in the portrait's pixels"
    );
    let (left, right) = (-2.0 * w, 3.0 * w);
    let edge = |offset: f64, at: f64| y + offset + slope * (at - x);
    let beam_lines = ".poster__lines line.poster__line--beam";
    assert_close(
        &attribute_numbers(&home, beam_lines, "y1"),
        &[edge(-width / 2.0, left), edge(width / 2.0, left)],
        "where the beam's edges start",
    );
    assert_close(
        &attribute_numbers(&home, beam_lines, "y2"),
        &[edge(-width / 2.0, right), edge(width / 2.0, right)],
        "where the beam's edges end",
    );
    let frame = front_matter_numbers("extra.frame_lines");
    assert_close(
        &attribute_numbers(&home, ".poster__lines line:not(.poster__line--beam)", "y1"),
        &[number(&frame, "eyes"), number(&frame, "nose")],
        "the straight lines at eye and nose height",
    );
    let stage = home.attributes(".poster", "style").join("");
    assert!(
        stage.contains(&format!("--face-x: {}", x / w)),
        "the face is placed from the beam: {stage}"
    );
}

/// The published portrait was lit with the beam in `content/_index.md`: run
/// `scripts/light-portrait` again whenever the beam or the cut-out changes.
#[test]
fn the_published_portrait_was_lit_with_the_current_beam() {
    let root = env!("CARGO_MANIFEST_DIR");
    let used =
        std::fs::read_to_string(format!("{root}/scripts/light-portrait/beam-used.toml")).unwrap();
    let used = numbers_after(&used, "");
    let beam = front_matter_numbers("extra.beam");
    for key in ["x", "y", "slope", "width", "soft"] {
        assert_eq!(
            number(&used, key),
            number(&beam, key),
            "{key}: the portrait was lit with another beam; run scripts/light-portrait"
        );
    }
    let published = std::fs::metadata(format!("{root}/static/home/portrait-lit.webp"))
        .unwrap()
        .len();
    assert_eq!(
        number(&used, "image_bytes"),
        published as f64,
        "static/home/portrait-lit.webp is not the file scripts/light-portrait wrote"
    );
}

/// `decoding="async"` on the portrait let Chromium paint the Poster without it and add it in a
/// later raster; with the software renderer that raster could be missed and the Poster showed no
/// face (PR #20 review). The portrait must decode with the page.
#[test]
fn the_portrait_is_not_decoded_asynchronously() {
    let home = home();
    assert_eq!(home.count("img.poster__portrait"), 1);
    assert_eq!(home.count("img.poster__portrait[decoding=async]"), 0);
    assert_eq!(home.count("img.poster__portrait[loading=lazy]"), 0);
}

#[test]
fn the_poster_captions_come_from_the_home_page_front_matter() {
    let root = env!("CARGO_MANIFEST_DIR");
    let front_matter = std::fs::read_to_string(format!("{root}/content/_index.md")).unwrap();
    let template = std::fs::read_to_string(format!("{root}/templates/index.html")).unwrap();
    let captions = home().texts(".poster__caption");
    assert_eq!(captions.len(), 3, "three captions on the Poster");
    for caption in captions {
        assert!(!caption.is_empty());
        assert!(
            front_matter.contains(&format!("\"{caption}\"")),
            "{caption:?} is held in content/_index.md"
        );
        assert!(
            !template.contains(&caption),
            "{caption:?} is not typed in the template"
        );
    }
}

#[test]
fn the_poster_keeps_its_corner_mark_and_thin_lines() {
    let home = home();
    // The fixtures hold five Videos.
    assert_eq!(home.texts(".poster__mark-number"), ["05"]);
    assert!(home.count(".poster svg.poster__lines line") >= 3);
}

/// The tools of the one list in `content/_index.md`, as (name, logo file) pairs, in order.
fn listed_tools() -> Vec<(String, String)> {
    let root = env!("CARGO_MANIFEST_DIR");
    let front_matter = std::fs::read_to_string(format!("{root}/content/_index.md")).unwrap();
    let value = |line: &str| line.split('"').nth(1).unwrap().to_owned();
    let mut tools = Vec::new();
    let mut lines = front_matter.lines();
    while let Some(line) = lines.next() {
        if line.trim() == "[[extra.tools]]" {
            let name = value(lines.next().expect("a tool has a name"));
            let logo = value(lines.next().expect("a tool has a logo"));
            tools.push((name, logo));
        }
    }
    tools
}

#[test]
fn the_tool_tiles_come_from_the_one_list_and_each_has_a_name() {
    let home = home();
    let tools = listed_tools();
    assert!(tools.len() >= 8, "at least the 8 tools a phone shows");

    let names: Vec<String> = tools.iter().map(|(name, _)| name.clone()).collect();
    assert_eq!(
        home.texts(".poster__tools .tool .tool__name"),
        names,
        "one tile per listed tool, in the list's order, each with its name"
    );
    assert_eq!(home.count(".poster__tools .tool"), tools.len());
    assert_eq!(
        home.count(".poster__tools[aria-label]"),
        1,
        "the group of tiles is named"
    );

    let logos = home.attributes(".poster__tools .tool img", "src");
    assert_eq!(logos.len(), tools.len(), "one logo per tile");
    for (address, (name, logo)) in logos.iter().zip(&tools) {
        let file = built_file(address);
        assert_eq!(file, format!("home/logos/{logo}"), "the logo of {name}");
        assert!(site().has_file(file), "{file} is in the built site");
        assert!(
            site().file(file).contains("<svg") && !site().file(file).contains("<script"),
            "{file} is a plain SVG"
        );
    }

    let template = std::fs::read_to_string(format!(
        "{}/templates/index.html",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap();
    for name in &names {
        assert!(
            !template.contains(name.as_str()),
            "{name} is not typed in the template"
        );
    }
}

#[test]
fn the_tool_tiles_are_not_links() {
    assert_eq!(home().count(".poster__tools a"), 0);
}

#[test]
fn the_home_page_links_to_the_three_other_pages() {
    let hrefs = home().attributes("main a", "href");
    for path in ["real-or-bs/", "playbooks/", "videos/"] {
        let expected = format!("{SUB_PATH_BASE_URL}/{path}");
        assert!(hrefs.contains(&expected), "home links to {expected}");
    }
}

#[test]
fn the_real_list_entry_point_shows_the_tally_and_links_to_real_or_bs() {
    let home = home();
    let real_or_bs = format!("{SUB_PATH_BASE_URL}/real-or-bs/");
    assert_eq!(
        home.attributes("a.entry-card--real", "href"),
        [real_or_bs],
        "the REAL list entry point"
    );
    assert_eq!(
        home.texts("a.entry-card--real .tally__count"),
        ["REAL: 2", "BS: 1"],
        "the Tally counts the fixture Verdicts"
    );
}

#[test]
fn the_playbooks_entry_point_counts_the_playbooks_and_links_to_them() {
    let home = home();
    let playbooks = format!("{SUB_PATH_BASE_URL}/playbooks/");
    assert_eq!(
        home.attributes("a.entry-card--playbooks", "href"),
        [playbooks],
        "the Playbooks entry point"
    );
    let count = home.texts("a.entry-card--playbooks .entry-card__count");
    let words: Vec<&str> = count[0].split_whitespace().collect();
    assert_eq!(words, ["2", "Playbooks"]);
}

#[test]
fn the_latest_video_is_the_newest_entry() {
    let home = home();
    assert_eq!(home.texts(".latest__title"), ["Voice Clone"]);
    assert_eq!(home.texts(".latest__format"), ["Real or BS"]);
    assert_eq!(
        home.texts(".latest__description"),
        ["Fixture: a Real or BS Video with the Verdict real and no Playbook."]
    );
    // No other Video is shown on the home page.
    let main_text = home.texts("main").join(" ");
    for older in [
        "Repo Radar",
        "Inbox Zero",
        "Robot Barista",
        "First Hackathon",
    ] {
        assert!(
            !main_text.contains(older),
            "{older} is not the latest Video"
        );
    }
}

#[test]
fn the_latest_video_links_only_where_it_was_posted() {
    // The newest fixture Video has an Instagram link and no TikTok link.
    assert_eq!(
        home().attributes(".latest__links a", "href"),
        ["https://www.instagram.com/reel/FIXTURE0003/"]
    );
    assert_eq!(home().texts(".latest__links a"), ["Watch on Instagram"]);
}

#[test]
fn the_latest_video_cover_is_resized_and_described() {
    let home = home();
    assert_eq!(
        home.attributes(".latest__cover img[alt]:not([alt=''])", "alt"),
        ["Cover of the Video Voice Clone"]
    );
    let covers = home.attributes(".latest__cover img[alt]:not([alt=''])", "src");
    let file = built_file(&covers[0]);
    assert!(file.starts_with("processed_images/"), "{file}");
    assert!(site().has_file(file), "{file} is in the built site");
}

#[test]
fn the_home_page_links_to_all_videos() {
    assert_eq!(
        home().attributes("a.latest__all", "href"),
        [format!("{SUB_PATH_BASE_URL}/videos/")]
    );
}

#[test]
fn stickers_are_decoration_and_come_resized_from_the_site_itself() {
    let home = home();
    let stickers = home.attributes("img.sticker", "src");
    assert!(!stickers.is_empty(), "taped-photo and sticker details");
    assert_eq!(
        home.count("img.sticker"),
        home.count("img.sticker[alt=''][aria-hidden=true]"),
        "stickers are hidden from screen readers"
    );
    for sticker in stickers {
        let file = built_file(&sticker);
        assert!(file.starts_with("processed_images/"), "{file}");
        assert!(site().has_file(file), "{file} is in the built site");
    }
}
