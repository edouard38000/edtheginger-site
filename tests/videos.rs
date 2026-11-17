//! The Videos page: every Video by Format, as covers that link out to TikTok and Instagram.

use std::sync::OnceLock;

use site_checks::{Page, Site, SUB_PATH_BASE_URL};

fn site() -> &'static Site {
    static SITE: OnceLock<Site> = OnceLock::new();
    SITE.get_or_init(Site::build)
}

fn videos() -> Page {
    site().page("videos/")
}

/// The platform links shown for one Video, as (platform, address) pairs in page order.
fn links_of(page: &Page, title: &str) -> Vec<(String, String)> {
    let selector = format!(r#".video a.video__link[aria-label^="Watch {title} on "]"#);
    page.texts(&selector)
        .into_iter()
        .zip(page.attributes(&selector, "href"))
        .collect()
}

fn link(platform: &str, address: &str) -> (String, String) {
    (platform.to_owned(), address.to_owned())
}

#[test]
fn the_videos_page_has_its_heading() {
    assert_eq!(videos().texts("main h1"), ["Videos"]);
}

#[test]
fn every_video_is_present_with_its_title_under_its_format_newest_first() {
    let page = videos();
    assert_eq!(page.count(".video"), 5, "the five fixture Videos");
    assert_eq!(
        page.texts(".videos__group-title"),
        ["Real or BS", "GitHub tools", "Storytelling"],
        "one group per Format that has a Video, in the order of the Formats"
    );
    assert_eq!(
        page.texts("#format-real-or-bs .video__title"),
        ["Voice Clone", "Inbox Zero", "Robot Barista"]
    );
    assert_eq!(
        page.texts("#format-github-tools .video__title"),
        ["Repo Radar"]
    );
    assert_eq!(
        page.texts("#format-storytelling .video__title"),
        ["First Hackathon"]
    );
}

#[test]
fn every_video_says_its_format() {
    let page = videos();
    for (group, format, count) in [
        ("format-real-or-bs", "Real or BS", 3),
        ("format-github-tools", "GitHub tools", 1),
        ("format-storytelling", "Storytelling", 1),
    ] {
        assert_eq!(
            page.texts(&format!("#{group} .video .video__format")),
            vec![format; count],
            "the Format shown on each Video of {format}"
        );
    }
}

#[test]
fn every_video_has_a_cover_resized_at_build_time() {
    let page = videos();
    let covers = page.attributes(".video .video__cover img", "src");
    assert_eq!(covers.len(), 5, "one cover per Video");
    for cover in covers {
        assert!(
            cover.starts_with(&format!("{SUB_PATH_BASE_URL}/processed_images/")),
            "{cover} is not a cover resized by the build"
        );
        let file = cover
            .strip_prefix(&format!("{SUB_PATH_BASE_URL}/"))
            .unwrap();
        assert!(site().has_file(file), "{file} is in the built site");
    }
    assert_eq!(
        page.attributes(".video .video__cover img", "width"),
        ["360"; 5],
        "covers are 360 px wide"
    );
    assert_eq!(
        page.attributes(".video .video__cover img", "height"),
        ["640"; 5],
        "covers stay 9:16"
    );
}

#[test]
fn a_video_with_both_links_shows_tiktok_and_instagram() {
    let page = videos();
    assert_eq!(
        links_of(&page, "Robot Barista"),
        [
            link(
                "TikTok",
                "https://www.tiktok.com/@fixture/video/1000000000000000001"
            ),
            link("Instagram", "https://www.instagram.com/reel/FIXTURE0001/"),
        ]
    );
    assert_eq!(
        links_of(&page, "Repo Radar"),
        [
            link(
                "TikTok",
                "https://www.tiktok.com/@fixture/video/1000000000000000004"
            ),
            link("Instagram", "https://www.instagram.com/reel/FIXTURE0004/"),
        ]
    );
}

#[test]
fn a_video_with_one_platform_link_shows_only_that_link() {
    let page = videos();
    assert_eq!(
        links_of(&page, "Inbox Zero"),
        [link(
            "TikTok",
            "https://www.tiktok.com/@fixture/video/1000000000000000002"
        )]
    );
    assert_eq!(
        links_of(&page, "Voice Clone"),
        [link(
            "Instagram",
            "https://www.instagram.com/reel/FIXTURE0003/"
        )]
    );
}

#[test]
fn a_video_without_a_link_shows_none() {
    let page = videos();
    assert!(links_of(&page, "First Hackathon").is_empty());
    // Six platform links in all: 2 + 1 + 1 + 2 + 0.
    assert_eq!(page.count(".video a.video__link"), 6);
    assert_eq!(page.count(".video[data-links='0']"), 1);
    assert_eq!(page.count(".video[data-links='0'] a"), 0);
}

#[test]
fn a_cover_links_to_the_video_only_when_it_has_a_link() {
    let page = videos();
    let covers = page.attributes(".video a.video__cover", "href");
    assert_eq!(covers.len(), 4, "the four Videos with a link");
    for cover in &covers {
        assert!(
            cover.starts_with("https://www.tiktok.com/")
                || cover.starts_with("https://www.instagram.com/"),
            "{cover} goes to TikTok or Instagram"
        );
    }
    // On the grid the cover repeats a labelled link, so it is kept out of the keyboard order.
    assert_eq!(page.count(".video a.video__cover[tabindex='-1']"), 4);
}

#[test]
fn there_is_a_way_to_see_one_format_at_a_time() {
    let page = videos();
    assert_eq!(
        page.texts("nav.videos__formats a"),
        ["All", "Real or BS", "GitHub tools", "Storytelling"],
        "a link per Format that has a Video; Talking head has none in the fixtures"
    );
    let addresses = page.attributes("nav.videos__formats a", "href");
    let here = format!("{SUB_PATH_BASE_URL}/videos/#");
    for address in &addresses[1..] {
        let group = address
            .strip_prefix(&here)
            .unwrap_or_else(|| panic!("{address} stays on the Videos page"));
        assert_eq!(
            page.count(&format!("section.videos__group#{group}")),
            1,
            "{address} points at a Format group of the page"
        );
    }
    assert_eq!(addresses[0], format!("{here}all-videos"));
    assert_eq!(page.count("#all-videos"), 1);
}

#[test]
fn nothing_embeds_a_player_and_the_only_script_is_the_sites_own() {
    let page = videos();
    assert_eq!(page.count("iframe, video, audio, embed, object"), 0);
    let scripts = page.attributes("script", "src");
    assert_eq!(page.count("script"), 1);
    assert_eq!(scripts.len(), 1, "the script is a file, not inline");
    assert!(
        scripts[0].starts_with(&format!("{SUB_PATH_BASE_URL}/js/videos.js")),
        "{} is served by the site itself",
        scripts[0]
    );
    assert_eq!(page.count("script[defer]"), 1);
    assert!(site().has_file("js/videos.js"));
}

#[test]
fn the_portrait_of_the_desktop_layout_is_resized_at_build_time() {
    let page = videos();
    let portrait = page.attributes(".videos__wallpaper img", "src");
    assert_eq!(portrait.len(), 1);
    assert!(portrait[0].starts_with(&format!("{SUB_PATH_BASE_URL}/processed_images/")));
    // Decoration only: hidden from assistive technology.
    assert_eq!(page.count(".videos__wallpaper[aria-hidden='true']"), 1);
}
