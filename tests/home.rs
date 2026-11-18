//! The home page, top to bottom: the Hero (video, name, tagline, links), the Wheel's slot, the
//! Playbook shelf, the Ed's Daily slot and My Story (the Poster: lit portrait, beam lines, tool
//! tiles, captions, title; then the story text or the coming-soon line).
//! Built from the fixture Entries: five Videos, and two Playbooks, "Prompt Pack" (2026-01-20, no
//! cover) and "Inbox Zero" (2026-01-12, with a cover).

use std::sync::OnceLock;

use site_checks::{Page, Site, SUB_PATH_BASE_URL};

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

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

/// The text of `[extra] <key> = "..."` in the home page's front matter.
fn front_matter_text(key: &str) -> String {
    let text = std::fs::read_to_string(format!("{ROOT}/content/_index.md")).unwrap();
    let line = text
        .lines()
        .find(|line| line.trim_start().starts_with(&format!("{key} =")))
        .unwrap_or_else(|| panic!("{key} = \"...\" in content/_index.md"));
    line.split('"').nth(1).unwrap().to_owned()
}

fn template() -> String {
    std::fs::read_to_string(format!("{ROOT}/templates/index.html")).unwrap()
}

// --- Section order ------------------------------------------------------------------------------

#[test]
fn the_sections_are_hero_wheel_playbooks_eds_daily_my_story() {
    let home = home();
    assert_eq!(
        home.count(
            "main > section.hero + .home-slot[data-slot=wheel] + section.shelf \
             + .home-slot[data-slot=eds-daily] + section.story"
        ),
        1,
        "Hero, the Wheel's slot, the Playbook shelf, the Ed's Daily slot, My Story, in that order"
    );
    assert_eq!(
        home.count("main > *"),
        5,
        "nothing else in the page: {:?}",
        home.attributes("main > *", "class")
    );
}

#[test]
fn the_entry_cards_and_the_latest_video_are_gone() {
    let home = home();
    assert_eq!(
        home.count(".entry-points, .entry-card, .latest, .tally__count"),
        0
    );
    let css = site().file("main.css");
    for gone in [".entry-card", ".entry-points", ".latest", ".sticker-tag"] {
        assert!(!css.contains(gone), "the unused style {gone} is removed");
    }
}

// --- The Hero ----------------------------------------------------------------------------------

#[test]
fn the_hero_name_is_the_one_h1_and_real_text() {
    let home = home();
    assert_eq!(home.count("h1"), 1);
    assert_eq!(home.count("section.hero h1"), 1, "the h1 is in the Hero");
    let title = home.texts(".hero h1").join(" ");
    let words: Vec<&str> = title.split_whitespace().collect();
    assert_eq!(words.join(" ").to_uppercase(), "EDOUARD THE GINGER");
    assert_eq!(
        home.texts(".hero h1 .hero__name"),
        ["Edouard", "the Ginger"],
        "EDOUARD left of the centre mark, THE GINGER right of it"
    );
    assert_eq!(
        home.count(
            ".hero h1 > .hero__name--first + svg.hero__mark[aria-hidden=true] + .hero__name--last"
        ),
        1
    );
}

#[test]
fn the_hero_tagline_is_text_held_in_the_front_matter() {
    let tagline = front_matter_text("tagline");
    assert!(!tagline.is_empty());
    assert_eq!(
        home().texts(".hero .hero__tagline"),
        std::slice::from_ref(&tagline)
    );
    assert!(
        !template().contains(&tagline),
        "the tagline is not typed in the template"
    );
}

#[test]
fn the_hero_video_is_muted_loops_plays_inline_and_has_a_poster() {
    let home = home();
    let video = "section.hero video.hero__video";
    assert_eq!(home.count(video), 1);
    for attribute in ["autoplay", "muted", "loop", "playsinline", "poster"] {
        assert_eq!(
            home.count(&format!("{video}[{attribute}]")),
            1,
            "the Hero video has {attribute}"
        );
    }
    assert_eq!(home.attributes(video, "preload"), ["metadata"]);
    assert_eq!(
        home.count(&format!("{video} track")),
        0,
        "no captions to read"
    );
    assert_eq!(
        home.count(&format!("{video}[controls]")),
        0,
        "no controls on the Hero"
    );
}

#[test]
fn the_hero_files_come_from_the_one_setting_and_are_published() {
    let home = home();
    let video_file = front_matter_text("video");
    let poster_file = front_matter_text("poster");
    let video = "section.hero video.hero__video";
    assert_eq!(
        home.attributes(&format!("{video} source"), "src"),
        [format!("{SUB_PATH_BASE_URL}/{video_file}")]
    );
    assert_eq!(
        home.attributes(video, "poster"),
        [format!("{SUB_PATH_BASE_URL}/{poster_file}")]
    );
    assert!(site().has_file(&video_file), "{video_file} is published");
    assert!(site().has_file(&poster_file), "{poster_file} is published");
    let template = template();
    for file in [&video_file, &poster_file] {
        assert!(
            !template.contains(file.as_str()),
            "{file} is not typed in the template: swapping the Hero needs no template change"
        );
    }
}

#[test]
fn the_hero_video_is_under_one_and_a_half_megabytes_and_the_poster_is_light() {
    let video = std::fs::metadata(format!("{ROOT}/static/{}", front_matter_text("video")))
        .unwrap()
        .len();
    assert!(video < 1_500_000, "the Hero video is {video} bytes");
    let poster = std::fs::metadata(format!("{ROOT}/static/{}", front_matter_text("poster")))
        .unwrap()
        .len();
    assert!(poster < 100_000, "the Hero poster is {poster} bytes");
}

#[test]
fn the_hero_video_has_no_audio_track_and_starts_fast() {
    let bytes = std::fs::read(format!("{ROOT}/static/{}", front_matter_text("video"))).unwrap();
    let find = |tag: &[u8]| bytes.windows(4).position(|window| window == tag);
    let moov = find(b"moov").expect("an MP4 with a moov box");
    let mdat = find(b"mdat").expect("an MP4 with an mdat box");
    assert!(moov < mdat, "fast-start: the index comes before the media");
    assert!(find(b"vide").is_some(), "a video track");
    assert!(find(b"soun").is_none(), "no audio track");
    assert!(find(b"avc1").is_some(), "H.264");
}

/// With "reduce motion" the video has no source to play, so it shows its poster and never plays,
/// with or without JavaScript.
#[test]
fn with_reduce_motion_the_hero_shows_its_poster_and_does_not_play() {
    let home = home();
    let sources = "section.hero video.hero__video source";
    assert_eq!(home.count(sources), 1, "one source");
    assert_eq!(
        home.attributes(sources, "media"),
        ["(prefers-reduced-motion: no-preference)"]
    );
}

#[test]
fn the_hero_poster_is_fetched_first() {
    let site = site();
    let page = site.file("index.html");
    let poster = format!("{SUB_PATH_BASE_URL}/{}", front_matter_text("poster"));
    let preload =
        format!("<link rel=\"preload\" as=\"image\" href=\"{poster}\" fetchpriority=\"high\">");
    assert!(page.contains(&preload), "the Hero poster is preloaded");
    assert_eq!(
        home().count("img[fetchpriority=high]"),
        0,
        "no other image competes with it"
    );
}

#[test]
fn the_hero_links_go_to_the_three_other_pages() {
    let home = home();
    assert_eq!(home.count(".hero nav.hero__links[aria-label]"), 1);
    assert_eq!(
        home.texts(".hero__links a"),
        ["Real or BS", "Playbooks", "Videos"]
    );
    assert_eq!(
        home.attributes(".hero__links a", "href"),
        ["real-or-bs/", "playbooks/", "videos/"].map(|path| format!("{SUB_PATH_BASE_URL}/{path}"))
    );
}

// --- The slots ---------------------------------------------------------------------------------

#[test]
fn the_wheel_and_eds_daily_slots_are_empty_for_now() {
    let home = home();
    for slot in ["wheel", "eds-daily"] {
        let selector = format!(".home-slot[data-slot={slot}]");
        assert_eq!(home.count(&selector), 1, "the {slot} slot");
        assert_eq!(
            home.count(&format!("{selector} *")),
            0,
            "the {slot} slot is empty"
        );
    }
}

// --- My Story ----------------------------------------------------------------------------------

#[test]
fn my_story_opens_with_the_poster_then_the_coming_soon_line() {
    let home = home();
    assert_eq!(home.count("section.story[aria-label]"), 1);
    assert_eq!(
        home.count("section.story > .poster + .story__text"),
        1,
        "the Poster, then the story text"
    );
    let title = home.texts(".story .poster h2.poster__title").join(" ");
    let words: Vec<&str> = title.split_whitespace().collect();
    assert_eq!(words.join(" ").to_uppercase(), "EDOUARD THE GINGER");
    let coming_soon = front_matter_text("story_coming_soon");
    assert_eq!(
        home.texts(".story__text .story__coming-soon"),
        std::slice::from_ref(&coming_soon)
    );
    assert!(
        !template().contains(&coming_soon),
        "the coming-soon line is not typed in the template"
    );
}

#[test]
fn my_story_shows_the_story_once_it_is_written() {
    let index = std::fs::read_to_string(format!("{ROOT}/content/_index.md")).unwrap();
    let with_story = format!("{index}\nI grew up somewhere. *Fixture story.*\n");
    let site = Site::try_build_with(&[("content/_index.md", with_story.as_bytes())])
        .unwrap_or_else(|message| panic!("{message}"));
    let home = site.page("");
    assert_eq!(home.count(".story__coming-soon"), 0, "no coming-soon line");
    assert_eq!(
        home.texts(".story__text p"),
        ["I grew up somewhere. Fixture story."]
    );
    assert_eq!(home.count(".story__text em"), 1, "the story is Markdown");
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

/// The width and height of a WebP file: lossy (VP8), lossless (VP8L) or extended (VP8X).
fn webp_size(bytes: &[u8]) -> (f64, f64) {
    assert_eq!(&bytes[0..4], b"RIFF");
    assert_eq!(&bytes[8..12], b"WEBP");
    let u16le = |at: usize| u32::from(bytes[at]) | u32::from(bytes[at + 1]) << 8;
    let u24 = |at: usize| u16le(at) | u32::from(bytes[at + 2]) << 16;
    match &bytes[12..16] {
        b"VP8X" => (f64::from(u24(24) + 1), f64::from(u24(27) + 1)),
        b"VP8 " => (f64::from(u16le(26) & 0x3fff), f64::from(u16le(28) & 0x3fff)),
        b"VP8L" => {
            let bits = u32::from_le_bytes([bytes[21], bytes[22], bytes[23], bytes[24]]);
            (
                f64::from((bits & 0x3fff) + 1),
                f64::from(((bits >> 14) & 0x3fff) + 1),
            )
        }
        other => panic!("unknown WebP chunk {other:?}"),
    }
}

/// The portrait the Poster shows: `portrait` in the home page's front matter, a path under static/.
fn portrait_file() -> String {
    let root = env!("CARGO_MANIFEST_DIR");
    let text = std::fs::read_to_string(format!("{root}/content/_index.md")).unwrap();
    let line = text
        .lines()
        .find(|line| line.trim_start().starts_with("portrait ="))
        .expect("portrait = \"...\" in content/_index.md");
    line.split('"').nth(1).unwrap().to_owned()
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

/// The lines are the edges of the light: the template draws them from `[extra.beam]`, measured
/// in the pixels of the portrait the Poster shows, so they sit on the light at every width.
#[test]
fn the_lines_are_the_edges_of_the_light_on_the_portrait() {
    let root = env!("CARGO_MANIFEST_DIR");
    let beam = front_matter_numbers("extra.beam");
    let (x, y, slope, width) = (
        number(&beam, "x"),
        number(&beam, "y"),
        number(&beam, "slope"),
        number(&beam, "width"),
    );
    let image = std::fs::read(format!("{root}/static/{}", portrait_file())).unwrap();
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
    assert_eq!(
        home.count(".poster__lines line"),
        2,
        "two lines, the edges of the light"
    );
    let stage = home.attributes(".poster", "style").join("");
    assert!(
        stage.contains(&format!("--face-x: {}", x / w)),
        "the face is placed from the beam: {stage}"
    );
}

/// The scripted light is only ever the portrait itself: with the AI relight in use, the page adds
/// no light of its own and does not publish the scripted image. With the scripted portrait in use,
/// it must have been lit with the current beam: run `scripts/light-portrait` after any change.
#[test]
fn the_scripted_light_is_used_only_with_the_scripted_portrait() {
    let root = env!("CARGO_MANIFEST_DIR");
    let scripted = "home/portrait-lit.webp";
    if portrait_file() != scripted {
        assert!(!site().has_file(scripted), "{scripted} is not published");
        assert_eq!(
            home().count(".poster__beam"),
            0,
            "no light added on the page"
        );
        return;
    }
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
    let published = std::fs::metadata(format!("{root}/static/{scripted}"))
        .unwrap()
        .len();
    assert_eq!(
        number(&used, "image_bytes"),
        published as f64,
        "static/{scripted} is not the file scripts/light-portrait wrote"
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
    assert_eq!(home.count(".poster svg.poster__lines line"), 2);
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

// --- The Playbook shelf ------------------------------------------------------------------------

#[test]
fn the_playbook_shelf_is_named_by_its_heading() {
    let home = home();
    let label = home.attributes("section.shelf", "aria-labelledby");
    assert_eq!(label.len(), 1, "the shelf is labelled");
    let heading = format!("section.shelf h2#{}", label[0]);
    assert_eq!(home.texts(&heading), ["Latest Playbooks"]);
    assert_eq!(
        home.count("section.shelf h1"),
        0,
        "the Hero's name stays the one h1"
    );
}

#[test]
fn the_playbook_shelf_lists_every_playbook_newest_first() {
    assert_eq!(
        home().texts(".shelf .shelf__card-title"),
        ["Prompt Pack", "Inbox Zero"]
    );
}

#[test]
fn each_grab_it_here_button_links_to_the_pdf_of_its_entry() {
    let home = home();
    assert_eq!(
        home.attributes(".shelf a.playbook-card__button", "href"),
        [
            "https://example.com/playbooks/prompt-pack.pdf",
            "https://example.com/playbooks/inbox-zero-playbook.pdf"
        ]
    );
    assert_eq!(
        home.attributes(".shelf a.playbook-card__button", "rel"),
        ["external", "external"]
    );
    let names: Vec<String> = home
        .texts(".shelf a.playbook-card__button")
        .iter()
        .map(|text| text.replace('\u{2192}', "").trim().to_owned())
        .collect();
    assert_eq!(
        names,
        [
            "Grab it here: the Prompt Pack Playbook (PDF)",
            "Grab it here: the Inbox Zero Playbook (PDF)"
        ],
        "screen readers hear which Playbook each button grabs"
    );
}

#[test]
fn the_shelf_shows_a_cover_or_the_title_block() {
    let home = home();
    // Inbox Zero has a cover: resized at build time, described, loaded lazily.
    let covers = ".shelf img.playbook-card__image";
    assert_eq!(
        home.attributes(covers, "alt"),
        ["Cover of the Inbox Zero Playbook"]
    );
    assert_eq!(home.count(&format!("{covers}[loading=lazy]")), 1);
    let file = built_file(&home.attributes(covers, "src")[0]).to_owned();
    assert!(file.starts_with("processed_images/"), "{file}");
    assert!(site().has_file(&file), "{file} is in the built site");
    // Prompt Pack has none: the orange title block, as on the Playbooks page.
    assert_eq!(
        home.texts(".shelf .playbook-card__cover--title .playbook-card__block-title"),
        ["Prompt Pack"]
    );
}

#[test]
fn the_newest_playbook_on_the_shelf_gets_the_new_pill() {
    let home = home();
    assert_eq!(home.count(".shelf .playbook-card__new"), 1);
    assert_eq!(
        home.count(".shelf__item:first-child .playbook-card__new"),
        1,
        "the pill is on the newest Playbook"
    );
}

#[test]
fn the_shelf_links_to_all_playbooks() {
    assert_eq!(
        home().attributes(".shelf a.shelf__all", "href"),
        [format!("{SUB_PATH_BASE_URL}/playbooks/")]
    );
}

/// A Playbook Entry dated `2026-02-<day>`, for the shelf checks.
fn playbook_entry(day: u32) -> String {
    format!(
        "+++\ntitle = \"Extra {day:02}\"\ndate = 2026-02-{day:02}\n\
         description = \"Fixture: one more Playbook.\"\n\n[extra]\n\
         pdf = \"https://example.com/playbooks/extra-{day:02}.pdf\"\n+++\n"
    )
}

#[test]
fn the_shelf_shows_at_most_the_six_newest_playbooks() {
    let entries: Vec<(String, String)> = (1..=5)
        .map(|day| {
            (
                format!("content/playbooks/extra-{day:02}.md"),
                playbook_entry(day),
            )
        })
        .collect();
    let files: Vec<(&str, &[u8])> = entries
        .iter()
        .map(|(path, text)| (path.as_str(), text.as_bytes()))
        .collect();
    let site = Site::try_build_with(&files).unwrap_or_else(|message| panic!("{message}"));
    assert_eq!(
        site.page("").texts(".shelf .shelf__card-title"),
        [
            "Extra 05",
            "Extra 04",
            "Extra 03",
            "Extra 02",
            "Extra 01",
            "Prompt Pack"
        ],
        "seven Playbooks, the six newest on the shelf"
    );
}

#[test]
fn a_site_with_no_playbooks_has_no_shelf() {
    // The fixture Video "Inbox Zero" names a Playbook, so it loses that line with them.
    let video = "videos/real-or-bs-02-inbox-zero.md";
    let text = std::fs::read_to_string(format!("{ROOT}/tests/fixtures/{video}")).unwrap();
    let without_playbook: String = text
        .lines()
        .filter(|line| !line.starts_with("playbook ="))
        .map(|line| format!("{line}\n"))
        .collect();
    let site = Site::try_build_changed(
        &[
            "content/playbooks/inbox-zero.md",
            "content/playbooks/prompt-pack.md",
        ],
        &[(&format!("content/{video}"), without_playbook.as_bytes())],
    )
    .unwrap_or_else(|message| panic!("{message}"));
    let home = site.page("");
    assert_eq!(home.count(".shelf"), 0, "no shelf");
    assert!(
        !home
            .texts("main h2")
            .iter()
            .any(|h| h == "Latest Playbooks"),
        "no empty heading"
    );
    assert_eq!(
        home.count("main > section.hero + .home-slot[data-slot=wheel] + .home-slot[data-slot=eds-daily] + section.story"),
        1,
        "the rest of the page is still there, in order"
    );
}
