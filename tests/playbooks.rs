//! The Playbooks page: every Playbook Entry, newest first, each with a button to its page or its
//! PDF. Then the Playbook pages themselves: a Playbook Entry with a body has a page of its own,
//! written with the components of `templates/components/playbook-page.html`.
//!
//! The fixture Playbook page lives in `tests/fixtures/playbook-pages/` and is added to the fixture
//! site only by the checks below, so the checks on what every Entry shares (tests/skeleton.rs) keep
//! building the PDF-only fixtures they expect.

use std::sync::OnceLock;

use site_checks::{Page, Site, SUB_PATH_BASE_URL};

/// What a fixture Playbook Entry says, read from its front matter.
struct Playbook {
    title: String,
    date: String,
    description: String,
    pdf: String,
    cover: Option<String>,
}

fn site() -> &'static Site {
    static SITE: OnceLock<Site> = OnceLock::new();
    SITE.get_or_init(Site::build)
}

fn page() -> Page {
    site().page("playbooks/")
}

/// The value of `key = ...` in an Entry's front matter, without its quotes.
fn value(entry: &str, key: &str) -> Option<String> {
    entry.lines().find_map(|line| {
        let (name, value) = line.split_once('=')?;
        (name.trim() == key).then(|| value.trim().trim_matches('"').to_owned())
    })
}

/// The fixture Playbook Entries, newest first.
fn fixture_playbooks() -> Vec<Playbook> {
    let folder = format!("{}/tests/fixtures/playbooks", env!("CARGO_MANIFEST_DIR"));
    let mut playbooks: Vec<Playbook> = std::fs::read_dir(folder)
        .unwrap()
        .map(|file| {
            let path = file.unwrap().path();
            let entry = std::fs::read_to_string(&path).unwrap();
            let required = |key: &str| {
                value(&entry, key).unwrap_or_else(|| panic!("{key} in {}", path.display()))
            };
            Playbook {
                title: required("title"),
                date: required("date"),
                description: required("description"),
                pdf: required("pdf"),
                cover: value(&entry, "cover"),
            }
        })
        .collect();
    assert!(playbooks.len() >= 2, "fixture Playbook Entries");
    playbooks.sort_by(|a, b| b.date.cmp(&a.date));
    playbooks
}

/// The selector of the nth card on the page, counting from 1.
fn card(position: usize) -> String {
    format!(".playbooks__list > li:nth-child({position})")
}

#[test]
fn the_playbooks_page_has_its_heading() {
    assert_eq!(page().texts("main h1"), ["Playbooks"]);
}

#[test]
fn every_playbook_is_on_the_page_newest_first() {
    let expected: Vec<String> = fixture_playbooks().into_iter().map(|p| p.title).collect();
    assert_eq!(page().texts(".playbooks__list > li h2"), expected);
}

#[test]
fn every_playbook_shows_what_it_helps_you_do() {
    let page = page();
    for (index, playbook) in fixture_playbooks().iter().enumerate() {
        assert_eq!(
            page.texts(&format!("{} p", card(index + 1))),
            std::slice::from_ref(&playbook.description),
            "the Playbook {:?}",
            playbook.title
        );
    }
}

#[test]
fn every_playbook_has_one_button_to_its_pdf_at_the_address_in_its_entry() {
    let page = page();
    for (index, playbook) in fixture_playbooks().iter().enumerate() {
        assert_eq!(
            page.attributes(&format!("{} a", card(index + 1)), "href"),
            std::slice::from_ref(&playbook.pdf),
            "the Playbook {:?}",
            playbook.title
        );
    }
}

#[test]
fn every_button_says_grab_it_here_and_names_its_playbook() {
    let page = page();
    for (index, playbook) in fixture_playbooks().iter().enumerate() {
        let texts = page.texts(&format!("{} a", card(index + 1)));
        let [text] = texts.as_slice() else {
            panic!("one button for the Playbook {:?}", playbook.title);
        };
        assert!(text.starts_with("Grab it here"), "{text:?}");
        assert!(
            text.contains(&playbook.title),
            "{text:?} names its Playbook"
        );
    }
}

#[test]
fn the_playbooks_are_a_list_with_one_item_each() {
    let page = page();
    let playbooks = fixture_playbooks().len();
    assert_eq!(page.count("main ul.playbooks__list"), 1);
    assert_eq!(page.count(".playbooks__list > li"), playbooks);
    assert_eq!(page.count(".playbooks__list > li > article h2"), playbooks);
}

#[test]
fn a_playbook_with_a_cover_shows_it_resized_and_described() {
    let page = page();
    let playbooks = fixture_playbooks();
    let with_cover: Vec<(usize, &Playbook)> = playbooks
        .iter()
        .enumerate()
        .filter(|(_, playbook)| playbook.cover.is_some())
        .collect();
    assert!(!with_cover.is_empty(), "a fixture Playbook with a cover");
    for (index, playbook) in with_cover {
        let image = format!("{} img.playbook-card__image", card(index + 1));
        assert_eq!(
            page.attributes(&image, "alt"),
            [format!("Cover of the {} Playbook", playbook.title)]
        );
        let sources = page.attributes(&image, "src");
        let [source] = sources.as_slice() else {
            panic!("one cover for the Playbook {:?}", playbook.title);
        };
        let resized = source
            .strip_prefix(&format!("{SUB_PATH_BASE_URL}/"))
            .unwrap_or_else(|| panic!("{source} is not served by the site itself"));
        assert!(
            resized.starts_with("processed_images/"),
            "{source} is resized"
        );
        assert!(site().has_file(resized), "{resized} is in the built site");
    }
}

#[test]
fn a_playbook_without_a_cover_gets_a_title_block_instead() {
    let page = page();
    let playbooks = fixture_playbooks();
    let without_cover: Vec<(usize, &Playbook)> = playbooks
        .iter()
        .enumerate()
        .filter(|(_, playbook)| playbook.cover.is_none())
        .collect();
    assert!(
        !without_cover.is_empty(),
        "a fixture Playbook without a cover"
    );
    for (index, playbook) in without_cover {
        let card = card(index + 1);
        assert_eq!(page.count(&format!("{card} img.playbook-card__image")), 0);
        assert_eq!(
            page.texts(&format!("{card} .playbook-card__block-title")),
            std::slice::from_ref(&playbook.title)
        );
    }
}

#[test]
fn every_image_on_the_page_has_an_alt_attribute() {
    let page = page();
    assert_eq!(page.count("main img"), page.count("main img[alt]"));
}

#[test]
fn no_pdf_is_copied_into_the_site() {
    fn pdfs(dir: &std::path::Path, found: &mut Vec<String>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                pdfs(&path, found);
            } else if path.extension().and_then(|e| e.to_str()) == Some("pdf") {
                found.push(path.display().to_string());
            }
        }
    }
    let mut found = Vec::new();
    pdfs(&site().output(), &mut found);
    assert!(found.is_empty(), "PDFs stay where they are: {found:?}");
}

// --- Playbook pages -----------------------------------------------------------------------------

/// The fixture Playbook page, as a path inside the Zola project.
const PAGE_ENTRY: &str = "content/playbooks/field-guide.md";

/// Its address under the base URL.
const PAGE_PATH: &str = "playbooks/field-guide/";

/// The fixture site plus the fixture Playbook page.
fn page_site() -> &'static Site {
    static SITE: OnceLock<Site> = OnceLock::new();
    SITE.get_or_init(|| {
        let entry = std::fs::read(format!(
            "{}/tests/fixtures/playbook-pages/field-guide.md",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
        Site::try_build_with(&[(PAGE_ENTRY, &entry)]).unwrap_or_else(|message| panic!("{message}"))
    })
}

fn playbook_page() -> Page {
    page_site().page(PAGE_PATH)
}

#[test]
fn a_playbook_with_a_body_and_no_pdf_has_its_own_page_at_its_address() {
    let page = playbook_page();
    let address = format!("{SUB_PATH_BASE_URL}/{PAGE_PATH}");
    assert_eq!(page.count("meta[http-equiv=refresh]"), 0, "no redirect");
    assert_eq!(
        page.attributes("link[rel=canonical]", "href"),
        std::slice::from_ref(&address)
    );
    assert_eq!(page.count("article.pb"), 1);
}

#[test]
fn the_playbook_page_has_one_h1_its_title_and_the_by_line() {
    let page = playbook_page();
    assert_eq!(page.texts("h1"), ["Fixture: the \"Field\" Guide"]);
    assert_eq!(page.texts(".pb__by"), ["by edouardtheginger"]);
    assert_eq!(
        page.attributes("a.pb__back", "href"),
        [format!("{SUB_PATH_BASE_URL}/playbooks/")]
    );
}

#[test]
fn the_playbook_page_has_its_meta_tags() {
    let page = playbook_page();
    let address = format!("{SUB_PATH_BASE_URL}/{PAGE_PATH}");
    assert_eq!(
        page.texts("title"),
        ["Fixture: the \"Field\" Guide | Edouard the Ginger"]
    );
    let description = "Fixture: a Playbook page that uses every component.";
    assert_eq!(
        page.attributes("meta[name=description]", "content"),
        [description]
    );
    assert_eq!(
        page.attributes("meta[property='og:title']", "content"),
        ["Fixture: the \"Field\" Guide"]
    );
    assert_eq!(
        page.attributes("meta[property='og:description']", "content"),
        [description]
    );
    assert_eq!(
        page.attributes("meta[property='og:url']", "content"),
        [address]
    );
    assert_eq!(
        page.attributes("meta[property='og:image']", "content"),
        [format!("{SUB_PATH_BASE_URL}/covers/fixture-cover.png")]
    );
}

#[test]
fn every_prompt_box_holds_the_exact_text_and_a_copy_button() {
    let page = playbook_page();
    assert_eq!(
        page.texts(".pb-prompt__text"),
        [
            "Find the best coffee in [YOUR CITY] & tell me \"why\", under [A PRICE] <today>.",
            "Open the app so I can pair my phone."
        ]
    );
    assert_eq!(page.texts(".pb-prompt .pb-label"), ["PROMPT 1", "PROMPT 2"]);
    assert_eq!(page.count(".pb-prompt"), 2);
    assert_eq!(page.count(".pb-prompt button[data-copy][type=button]"), 2);
    // Hidden until the script shows it, so it is harmless without JavaScript.
    assert_eq!(page.count(".pb-prompt button[data-copy][hidden]"), 2);
    assert_eq!(page.texts(".pb-prompt button"), ["Copy", "Copy"]);
    let scripts = page.attributes("script[src]", "src");
    assert_eq!(scripts.len(), 1);
    assert!(
        scripts[0].starts_with(&format!("{SUB_PATH_BASE_URL}/js/playbook.js")),
        "{scripts:?}"
    );
    assert!(page_site().has_file("js/playbook.js"));
}

#[test]
fn a_prompt_says_where_to_paste_it_and_shows_its_example_apart() {
    let page = playbook_page();
    assert_eq!(
        page.texts(".pb-prompt__dest"),
        ["paste in ChatGPT or Claude", "paste in Claude Code"]
    );
    assert_eq!(page.count(".pb-prompt:nth-of-type(1) .pb-prompt__logo"), 2);
    assert_eq!(
        page.texts(".pb-prompt__example"),
        ["One of mine: Find a cafe in Lyon."]
    );
}

#[test]
fn the_placeholders_of_a_prompt_are_wrapped_to_show_in_orange() {
    assert_eq!(
        playbook_page().texts(".pb-prompt__text .pb-hot"),
        ["[YOUR CITY]", "[A PRICE]"]
    );
}

#[test]
fn a_skill_shows_its_name_and_a_link_only_when_it_has_one() {
    let page = playbook_page();
    assert_eq!(
        page.texts(".pb-skill__name"),
        ["fixture-skill", "no-link-skill"]
    );
    assert_eq!(
        page.attributes(".pb-skill a", "href"),
        ["https://example.com/fixture-skill.zip"]
    );
    assert!(page.texts(".pb-skill a")[0].starts_with("GRAB IT HERE"));
}

#[test]
fn the_screenshots_are_resized_described_and_lazy() {
    let page = playbook_page();
    let images = ".pb-figure__image, .pb-shot__image";
    assert_eq!(page.count(images), 3);
    assert_eq!(page.count(&format!(":is({images})[loading=lazy]")), 3);
    assert_eq!(
        page.attributes(images, "alt"),
        [
            "Fixture: the wide screenshot.",
            "Fixture: the first phone screen.",
            "Fixture: the second phone screen."
        ]
    );
    for source in page.attributes(images, "src") {
        let file = source
            .strip_prefix(&format!("{SUB_PATH_BASE_URL}/"))
            .unwrap_or_else(|| panic!("{source} is not served by the site itself"));
        assert!(file.starts_with("processed_images/"), "{source} is resized");
        assert!(page_site().has_file(file), "{file} is in the built site");
    }
    assert_eq!(page.texts(".pb-shot .pb-label"), ["01 ONE", "02 TWO"]);
    assert_eq!(page.texts(".pb-figure__caption"), ["Fixture caption."]);
    // Every image has an alt; the stickers are decoration.
    assert_eq!(page.count("main img"), page.count("main img[alt]"));
}

#[test]
fn the_parts_are_headings_with_the_anchors_the_list_links_to() {
    let page = playbook_page();
    assert_eq!(
        page.texts(".pb-part h2"),
        ["Part 1: Start here", "Part 2: Finish"]
    );
    assert_eq!(
        page.attributes("header.pb-part", "id"),
        ["part-1", "part-2"]
    );
    let links = page.attributes(".pb__body > ol a", "href");
    assert_eq!(links.len(), 2);
    for (link, part) in links.iter().zip(["#part-1", "#part-2"]) {
        assert!(link.ends_with(part), "{link} links to {part}");
    }
    assert_eq!(
        page.texts("h2.pb-subhead"),
        ["What you need", "The two parts"]
    );
    assert_eq!(page.texts("h3.pb-subhead"), ["On your computer"]);
}

#[test]
fn steps_rows_labels_and_link_buttons_are_shown() {
    let page = playbook_page();
    assert_eq!(page.texts(".pb-step__n"), ["01", "02"]);
    assert_eq!(
        page.texts(".pb-step__head strong"),
        ["Say \"hello\"", "Paste the prompt"]
    );
    assert_eq!(page.texts(".pb-step__body strong"), ["hello"]);
    assert_eq!(page.texts(".pb-row__key"), ["A phone"]);
    assert_eq!(page.texts(".pb-label--block"), ["THIS ERASES EVERYTHING"]);
    assert_eq!(
        page.attributes("a.pb-button", "href"),
        [
            "https://example.com/free-week",
            "https://example.com/github"
        ]
    );
    assert_eq!(page.texts(".pb-note"), ["Fixture note."]);
}

#[test]
fn a_card_links_to_the_page_of_a_page_playbook_and_to_the_pdf_of_a_pdf_only_one() {
    let site = page_site();
    let page_address = format!("{SUB_PATH_BASE_URL}/{PAGE_PATH}");
    for (listing, card) in [
        ("playbooks/", ".playbooks__list > li"),
        ("", ".shelf__item"),
    ] {
        let page = site.page(listing);
        // The Playbook page is the newest fixture Playbook, so it comes first.
        let first = format!("{card}:first-child a.playbook-card__button");
        assert_eq!(
            page.attributes(&first, "href"),
            std::slice::from_ref(&page_address),
            "{listing:?}"
        );
        assert!(
            page.attributes(&first, "rel").is_empty(),
            "no rel=external on an internal link ({listing:?})"
        );
        let text = page.texts(&first).join("");
        assert!(text.starts_with("Grab it here"), "{text:?}");
        assert!(
            text.contains("the Fixture: the \"Field\" Guide Playbook") && !text.contains("PDF"),
            "{text:?}"
        );
        // The PDF-only Playbooks keep their PDF button.
        assert_eq!(
            page.attributes(
                &format!("{card}:not(:first-child) a.playbook-card__button"),
                "href"
            ),
            [
                "https://example.com/playbooks/prompt-pack.pdf",
                "https://example.com/playbooks/inbox-zero-playbook.pdf"
            ],
            "{listing:?}"
        );
    }
    assert_eq!(
        site.page("playbooks/")
            .texts(".playbooks__list > li:first-child .playbook-card__tag"),
        ["Playbook", "Online", "Jan 2026"]
    );
}

#[test]
fn pdf_only_playbooks_and_videos_still_send_the_visitor_to_their_listing() {
    let site = page_site();
    for (entry, listing) in [
        ("playbooks/prompt-pack/", "playbooks/"),
        ("playbooks/inbox-zero/", "playbooks/"),
        ("videos/real-or-bs-02-inbox-zero/", "videos/"),
        ("videos/github-tools-repo-radar/", "videos/"),
    ] {
        let target = format!("{SUB_PATH_BASE_URL}/{listing}");
        let page = site.page(entry);
        assert_eq!(
            page.attributes("meta[http-equiv=refresh]", "content"),
            [format!("0; url={target}")],
            "{entry}"
        );
        assert_eq!(page.count("article.pb"), 0, "{entry}");
    }
}
