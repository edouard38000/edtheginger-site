//! The Real or BS page: the Tally and the REAL list, computed from the Verdicts of the fixture
//! Video Entries (episode 1 bs, episode 2 real with a Playbook, episode 3 real without).

use std::sync::OnceLock;

use site_checks::{Page, Site};

fn page() -> Page {
    static SITE: OnceLock<Site> = OnceLock::new();
    SITE.get_or_init(Site::build).page("real-or-bs/")
}

/// How many fixture Video Entries carry this Verdict, read from the Entries themselves.
fn fixture_verdicts(verdict: &str) -> usize {
    let folder = format!("{}/tests/fixtures/videos", env!("CARGO_MANIFEST_DIR"));
    let line = format!("verdict = \"{verdict}\"");
    std::fs::read_dir(folder)
        .unwrap()
        .map(|file| std::fs::read_to_string(file.unwrap().path()).unwrap())
        .filter(|entry| entry.lines().any(|l| l.trim() == line))
        .count()
}

/// The cards of the REAL list, without the teaser.
const ITEMS: &str = ".real-list__item:not(.real-list__item--teaser)";

#[test]
fn the_real_or_bs_page_has_its_heading() {
    assert_eq!(page().texts("main h1"), ["Real or BS"]);
    assert_eq!(page().texts("main h2"), ["The REAL list"]);
}

#[test]
fn the_tally_equals_the_verdict_counts() {
    let (real, bs) = (fixture_verdicts("real"), fixture_verdicts("bs"));
    assert_eq!((real, bs), (2, 1), "the fixtures hold both Verdicts");
    let page = page();
    assert_eq!(
        page.texts(".tally span"),
        [format!("REAL: {real}"), format!("BS: {bs}")]
    );
    assert_eq!(
        page.attributes(".tally", "aria-label"),
        [format!("Tally: {real} real, {bs} BS")]
    );
}

#[test]
fn bs_videos_never_appear_in_the_real_list() {
    let page = page();
    assert_eq!(page.count(ITEMS), fixture_verdicts("real"));
    for item in page.texts(".real-list__item") {
        assert!(!item.contains("Robot Barista"), "a bs Video is listed");
        assert!(!item.contains("flat white"), "a bs Video is listed");
    }
}

#[test]
fn the_real_list_is_numbered_in_posting_order_and_shown_newest_first() {
    let page = page();
    // The teaser takes the next number; then Voice Clone (posted last) is 2, Inbox Zero is 1.
    assert_eq!(page.texts(".real-list__number"), ["3", "2", "1"]);
    assert_eq!(
        page.texts(".real-list__tested"),
        [
            "Cloning a voice from ten seconds of sound",
            "An agent that clears an inbox by itself"
        ]
    );
}

#[test]
fn a_teaser_for_the_next_number_sits_above_the_newest_item() {
    let page = page();
    assert_eq!(page.count("ol.real-list > li"), 3);
    assert_eq!(
        page.texts("ol.real-list > li:first-child .real-list__number"),
        ["3"]
    );
    assert_eq!(
        page.texts("ol.real-list > li:first-child .real-list__teaser"),
        ["To be found..."]
    );
    assert_eq!(page.count(".real-list__item--teaser"), 1);
}

#[test]
fn each_item_shows_what_was_tested_the_episode_and_a_description() {
    let page = page();
    assert_eq!(
        page.texts(&format!("{ITEMS} h3")),
        page.texts(".real-list__tested")
    );
    assert_eq!(
        page.texts(".real-list__episode"),
        ["Episode 3, Voice Clone", "Episode 2, Inbox Zero"]
    );
    assert_eq!(
        page.texts(".real-list__description"),
        [
            "Fixture: a Real or BS Video with the Verdict real and no Playbook.",
            "Fixture: a Real or BS Video with the Verdict real and a Playbook."
        ]
    );
}

#[test]
fn an_item_links_to_its_playbook_pdf_only_when_it_references_one() {
    let page = page();
    // Inbox Zero (third in the list) references a Playbook; Voice Clone (second) does not.
    assert_eq!(
        page.attributes("ol.real-list > li:nth-child(3) a", "href"),
        ["https://example.com/playbooks/inbox-zero-playbook.pdf"]
    );
    assert_eq!(page.count("ol.real-list > li:nth-child(2) a"), 0);
    assert_eq!(page.count("ol.real-list a"), 1);
    let label = page.texts("ol.real-list a").join("");
    assert!(label.starts_with("Grab the Playbook"), "{label}");
}

#[test]
fn the_page_explains_the_series_and_invites_viewers_to_tag_edouard() {
    let page = page();
    let intro = page.texts(".real-or-bs__intro").join("");
    assert!(intro.contains("I test viral AI claims"), "{intro}");
    let invite = page.texts(".real-or-bs__invite").join("");
    assert!(invite.contains("Tag @edouardtheginger"), "{invite}");
}

#[test]
fn stickers_are_decoration_and_served_resized() {
    let page = page();
    assert_eq!(page.attributes("main img", "alt"), ["", ""]);
    for src in page.attributes("main img", "src") {
        assert!(src.contains("/processed_images/"), "{src} is not resized");
    }
}

#[test]
fn an_item_links_to_the_page_of_its_playbook_when_the_playbook_is_a_page() {
    let root = env!("CARGO_MANIFEST_DIR");
    let video = "videos/real-or-bs-02-inbox-zero.md";
    let text = std::fs::read_to_string(format!("{root}/tests/fixtures/{video}")).unwrap();
    let pointing_at_the_page = text.replace(
        "playbook = \"playbooks/inbox-zero.md\"",
        "playbook = \"playbooks/field-guide.md\"",
    );
    let page_entry = std::fs::read(format!(
        "{root}/tests/fixtures/playbook-pages/field-guide.md"
    ))
    .unwrap();
    let site = Site::try_build_with(&[
        (&format!("content/{video}"), pointing_at_the_page.as_bytes()),
        ("content/playbooks/field-guide.md", &page_entry),
    ])
    .unwrap_or_else(|message| panic!("{message}"));
    let page = site.page("real-or-bs/");
    assert_eq!(
        page.attributes("ol.real-list a", "href"),
        [format!(
            "{}/playbooks/field-guide/",
            site_checks::SUB_PATH_BASE_URL
        )]
    );
    let label = page.texts("ol.real-list a").join("");
    assert!(label.starts_with("Grab the Playbook"), "{label}");
    assert!(!label.contains("PDF"), "{label}");
}
