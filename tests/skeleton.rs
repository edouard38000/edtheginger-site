//! What every page shares: the four pages, the Dock, the brand tokens, the fonts and the base URL.
//! Page tickets add their checks to their own file (`home.rs`, `real_or_bs.rs`, `playbooks.rs`,
//! `videos.rs`), not here.

use std::sync::OnceLock;

use site_checks::{Page, Site, PAGES, SUB_PATH_BASE_URL};

fn site() -> &'static Site {
    static SITE: OnceLock<Site> = OnceLock::new();
    SITE.get_or_init(Site::build)
}

fn every_html_page(site: &Site) -> Vec<Page> {
    site.html_files()
        .iter()
        .map(|file| Page::from_file(site, file))
        .collect()
}

#[test]
fn the_four_pages_exist() {
    for path in PAGES {
        assert!(
            site().has_file(&format!("{path}index.html")),
            "page {path:?} was not generated"
        );
    }
}

#[test]
fn the_dock_is_on_every_page_with_labelled_links() {
    let pages = every_html_page(site());
    assert!(pages.len() >= PAGES.len());
    for page in pages {
        let dock = page.dock_links();
        let expected = [
            (
                "https://www.instagram.com/edouardtheginger/",
                "Edouard the Ginger on Instagram",
            ),
            (
                "https://www.tiktok.com/@edouardtheginger",
                "Edouard the Ginger on TikTok",
            ),
            (
                "mailto:edouard@vesara.ai",
                "Email Edouard at edouard@vesara.ai",
            ),
        ];
        let found: Vec<(&str, &str)> = dock
            .iter()
            .map(|(href, label)| (href.as_str(), label.as_str()))
            .collect();
        assert_eq!(found, expected, "the Dock on {:?}", page.path());
        assert_eq!(
            page.count("nav.dock[aria-label]"),
            1,
            "one labelled Dock on {:?}",
            page.path()
        );
    }
}

#[test]
fn the_dock_values_are_held_once_in_the_site_config() {
    let root = env!("CARGO_MANIFEST_DIR");
    let config = std::fs::read_to_string(format!("{root}/config.toml")).unwrap();
    for value in [
        "https://www.instagram.com/edouardtheginger/",
        "https://www.tiktok.com/@edouardtheginger",
        "edouard@vesara.ai",
    ] {
        assert_eq!(config.matches(value).count(), 1, "{value} in config.toml");
    }
    let dock = std::fs::read_to_string(format!("{root}/templates/partials/dock.html")).unwrap();
    assert!(!dock.contains("instagram.com") && !dock.contains("tiktok.com"));
    assert!(!dock.contains("vesara.ai"));
}

#[test]
fn the_home_page_links_to_the_three_other_pages() {
    let hrefs = site().page("").attributes("main a", "href");
    for path in &PAGES[1..] {
        let expected = format!("{SUB_PATH_BASE_URL}/{path}");
        assert!(hrefs.contains(&expected), "home links to {expected}");
    }
}

#[test]
fn the_stylesheet_holds_the_brand_tokens() {
    let css = site().file("main.css").to_lowercase();
    for (token, value) in [
        ("--color-background", "#faf8f5"),
        ("--color-orange", "#c63b00"),
        ("--color-yellow", "#ffb926"),
        ("--color-charcoal", "#191919"),
        ("--color-muted", "#665f58"),
        ("--color-border", "#d6cdc3"),
        ("--color-pale-fill", "#f3f0eb"),
    ] {
        let declared =
            css.contains(&format!("{token}:{value}")) || css.contains(&format!("{token}: {value}"));
        assert!(declared, "{token} is {value}");
    }
}

#[test]
fn montserrat_is_self_hosted() {
    let css = site().file("main.css");
    for font in ["fonts/Montserrat-Bold.ttf", "fonts/Montserrat-Black.ttf"] {
        assert!(site().has_file(font), "{font} is in the built site");
        assert!(
            css.contains(&format!("url(\"{font}\")")),
            "{font} is loaded relative to main.css"
        );
    }
    assert!(
        !css.contains("http://") && !css.contains("https://") && !css.contains("url(\"//"),
        "the stylesheet loads nothing from another host"
    );
}

#[test]
fn pages_load_no_script_and_nothing_from_other_hosts() {
    for page in every_html_page(site()) {
        assert_eq!(page.count("script"), 0, "no script on {:?}", page.path());
        assert_eq!(page.count("iframe"), 0, "no iframe on {:?}", page.path());
        let mut loaded = page.attributes("link[rel=stylesheet], link[rel=preload]", "href");
        loaded.extend(page.attributes("img, source, video, audio", "src"));
        for address in loaded {
            assert!(
                address.starts_with(&format!("{SUB_PATH_BASE_URL}/")),
                "{address} on {:?} is not served by the site itself",
                page.path()
            );
        }
    }
}

/// Internal links and assets follow the one base URL setting: under a sub-path and at the root.
#[test]
fn internal_links_follow_the_base_url() {
    let root_site = Site::build_at("https://example.com");
    for built in [site(), &root_site] {
        let base = format!("{}/", built.base_url());
        for page in every_html_page(built) {
            let mut addresses = page.attributes("header a, main a", "href");
            addresses.extend(page.attributes("link[href]", "href"));
            addresses.extend(page.attributes("img", "src"));
            assert!(!addresses.is_empty());
            // The site's own host: the base URL up to its path.
            let origin: String = base.splitn(4, '/').take(3).collect::<Vec<_>>().join("/");
            for address in addresses {
                // Links out (Playbook PDFs, TikTok, Instagram) live on other hosts and are
                // used exactly as the Entry gives them. Everything else must follow the base URL.
                let links_out = address.starts_with("https://")
                    && address != origin
                    && !address.starts_with(&format!("{origin}/"));
                assert!(
                    address.starts_with(&base) || links_out,
                    "{address} on {:?} does not start with {base}",
                    page.path()
                );
            }
        }
    }
}

#[test]
fn every_fixture_entry_is_built_and_sends_the_visitor_to_its_page() {
    for (folder, listing) in [("videos", "videos/"), ("playbooks", "playbooks/")] {
        let fixtures = format!("{}/tests/fixtures/{folder}", env!("CARGO_MANIFEST_DIR"));
        let mut entries = 0;
        for file in std::fs::read_dir(fixtures).unwrap() {
            let name = file.unwrap().file_name().into_string().unwrap();
            let slug = name.strip_suffix(".md").expect("an Entry is a .md file");
            let page = site().page(&format!("{folder}/{slug}/"));
            let target = format!("{SUB_PATH_BASE_URL}/{listing}");
            assert_eq!(
                page.attributes("link[rel=canonical]", "href"),
                std::slice::from_ref(&target),
                "Entry {name}"
            );
            assert_eq!(
                page.attributes("meta[http-equiv=refresh]", "content"),
                [format!("0; url={target}")],
                "Entry {name}"
            );
            entries += 1;
        }
        assert!(entries >= 2, "fixture {folder} Entries");
    }
}

#[test]
fn the_real_entries_are_not_in_the_fixture_site() {
    for real in [
        "videos/real-or-bs-02-phone-use/index.html",
        "videos/real-or-bs-01-higgsfield/index.html",
        "playbooks/phone-use/index.html",
        "covers/placeholder-cover.png",
    ] {
        assert!(!site().has_file(real), "{real} is real content");
    }
}
