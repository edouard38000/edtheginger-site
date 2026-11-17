//! The Playbooks page: every Playbook Entry, newest first, each with a button to its PDF.

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
