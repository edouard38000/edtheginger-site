//! Ed's Daily: the pages under /daily/ where the signup endpoint and the confirmation email send the
//! visitor (`/daily/`, `check-your-inbox`, `confirmed`, `link-expired`), and the signup form that
//! `/daily/` and `link-expired` carry. The home page's form is checked in `home.rs`.

use std::sync::OnceLock;

use site_checks::{Page, Site, SUB_PATH_BASE_URL};

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn site() -> &'static Site {
    static SITE: OnceLock<Site> = OnceLock::new();
    SITE.get_or_init(Site::build)
}

/// `[extra] eds_daily_subscribe` in config.toml.
fn subscribe_address() -> String {
    let config = std::fs::read_to_string(format!("{ROOT}/config.toml")).unwrap();
    let line = config
        .lines()
        .find(|line| line.starts_with("eds_daily_subscribe ="))
        .expect("eds_daily_subscribe = \"...\" in config.toml");
    line.split('"').nth(1).unwrap().to_owned()
}

/// The addresses the signup contract redirects to: they must exist.
const PAGES: [&str; 4] = [
    "daily/",
    "daily/check-your-inbox/",
    "daily/confirmed/",
    "daily/link-expired/",
];

const FORM: &str = "form.eds-daily-form";

/// The one signup form of a page posts the email, the honeypot and its source to the endpoint.
fn assert_signup_form(page: &Page, source: &str) {
    let path = page.path();
    assert_eq!(page.count(FORM), 1, "one form on {path}");
    assert_eq!(page.attributes(FORM, "method"), ["post"], "{path}");
    assert_eq!(
        page.attributes(FORM, "action"),
        [subscribe_address()],
        "{path}"
    );
    assert_eq!(
        page.attributes(&format!("{FORM} input"), "name"),
        ["email", "company", "source"],
        "the fields of the form on {path}, and nothing else"
    );
    let email = format!("{FORM} input[name=email]");
    assert_eq!(
        page.count(&format!(
            "{email}[type=email][autocomplete=email][maxlength='254'][required]"
        )),
        1,
        "{path}"
    );
    let id = page
        .attributes(&email, "id")
        .pop()
        .expect("the field has an id");
    assert_eq!(
        page.texts(&format!("{FORM} label[for='{id}']")),
        ["Your email"],
        "a visible label on {path}"
    );
    assert_eq!(
        page.count(&format!(
            "{FORM} [aria-hidden=true] input[name=company][type=text][tabindex='-1'][autocomplete=off]"
        )),
        1,
        "the honeypot on {path}"
    );
    assert_eq!(
        page.attributes(&format!("{FORM} input[name=source][type=hidden]"), "value"),
        [source],
        "{path}"
    );
    assert_eq!(
        page.texts(&format!("{FORM} button[type=submit]")),
        ["Subscribe"],
        "{path}"
    );
    assert_eq!(
        page.texts(&format!("{FORM} [aria-live=polite]")),
        [""],
        "an empty live region for the messages on {path}"
    );
}

fn assert_script_loaded_once(page: &Page, loaded: bool) {
    let scripts = page.attributes("script[src*='/js/eds-daily.js']", "src");
    let deferred = page.count("script[src*='/js/eds-daily.js'][defer]");
    if loaded {
        assert_eq!(scripts.len(), 1, "loaded once on {}", page.path());
        assert_eq!(deferred, 1, "with defer on {}", page.path());
        assert!(scripts[0].starts_with(&format!("{SUB_PATH_BASE_URL}/js/eds-daily.js?h=")));
    } else {
        assert!(scripts.is_empty(), "no form, no script on {}", page.path());
    }
}

#[test]
fn the_four_daily_pages_exist() {
    for path in PAGES {
        assert!(
            site().has_file(&format!("{path}index.html")),
            "{path} is built"
        );
        let page = site().page(path);
        assert_eq!(page.count("main h1"), 1, "one title on {path}");
    }
}

#[test]
fn daily_and_link_expired_carry_the_form_with_the_archive_source() {
    for path in ["daily/", "daily/link-expired/"] {
        let page = site().page(path);
        assert_signup_form(&page, "daily-archive");
        assert_script_loaded_once(&page, true);
    }
}

#[test]
fn check_your_inbox_and_confirmed_link_home_and_have_no_form() {
    for path in ["daily/check-your-inbox/", "daily/confirmed/"] {
        let page = site().page(path);
        assert_eq!(page.count("form"), 0, "{path}");
        assert_script_loaded_once(&page, false);
        assert_eq!(
            page.attributes("main .eds-daily__back a", "href"),
            [format!("{SUB_PATH_BASE_URL}/")],
            "{path} links back to the home page"
        );
    }
}

#[test]
fn the_daily_titles_and_text() {
    let titles = |path| site().page(path).texts("main h1");
    assert_eq!(titles("daily/"), ["Ed's Daily"]);
    assert_eq!(
        site()
            .page("daily/")
            .texts("main header.eds-daily__header > h1 + p.eds-daily__tagline"),
        ["My newsletter, in your inbox every day."],
        "/daily/ has the home section's header: the title and the tagline box"
    );
    assert_eq!(titles("daily/check-your-inbox/"), ["Check your inbox"]);
    assert_eq!(titles("daily/confirmed/"), ["You're in"]);
    assert_eq!(titles("daily/link-expired/"), ["This link has expired"]);
    assert!(site()
        .page("daily/check-your-inbox/")
        .texts("main p")
        .iter()
        .any(|text| text.contains("Click the link in it to confirm")));
}

/// The pages the visitor only reaches from a signup stay out of search results; /daily/ does not.
#[test]
fn only_daily_itself_is_indexed() {
    assert_eq!(site().page("daily/").count("meta[name=robots]"), 0);
    for path in &PAGES[1..] {
        assert_eq!(
            site().page(path).attributes("meta[name=robots]", "content"),
            ["noindex"],
            "{path}"
        );
    }
}

/// The endpoint address is held once, in config.toml, and never typed in a template.
#[test]
fn the_endpoint_is_one_site_setting() {
    let address = subscribe_address();
    assert_eq!(address, "https://daily.edtheginger.com/subscribe");
    let config = std::fs::read_to_string(format!("{ROOT}/config.toml")).unwrap();
    assert_eq!(config.matches(&address).count(), 1);
    for entry in std::fs::read_dir(format!("{ROOT}/templates/partials"))
        .unwrap()
        .chain(std::fs::read_dir(format!("{ROOT}/templates")).unwrap())
    {
        let path = entry.unwrap().path();
        if path.is_file() {
            let text = std::fs::read_to_string(&path).unwrap();
            assert!(
                !text.contains("daily.edtheginger.com"),
                "{} types the endpoint",
                path.display()
            );
        }
    }
}

/// What static/js/eds-daily.js does, as far as its text shows: it sends the form with fetch as
/// JSON-accepting, cookie-less form data, maps the contract's answers to the form's messages,
/// disables the button while sending, and reads ?error= on /daily/. Its behaviour in a browser
/// is checked by hand (see the pull request).
#[test]
fn the_script_follows_the_signup_contract() {
    let script = std::fs::read_to_string(format!("{ROOT}/static/js/eds-daily.js")).unwrap();
    for needed in [
        "fetch(form.action",
        "Accept: \"application/json\"",
        "credentials: \"omit\"",
        "new URLSearchParams(new FormData(form))",
        "event.preventDefault()",
        "button.disabled = true",
        "button.disabled = false",
        "response.status === 202",
        "response.status === 400",
        "response.status === 429",
        "\"invalid_email\"",
        "\"rate_limited\"",
        "\"unavailable\"",
        ".get(\"error\")",
        "status.textContent",
    ] {
        assert!(script.contains(needed), "eds-daily.js: {needed}");
    }
    assert!(!script.contains("innerHTML"), "messages are set as text");
    assert!(!script.contains("location.href ="), "never leaves the page");
    let form =
        std::fs::read_to_string(format!("{ROOT}/templates/partials/eds-daily-form.html")).unwrap();
    for (answer, message) in [
        ("pending", "Check your inbox to confirm."),
        ("invalid_email", "That email doesn't look right."),
        ("rate_limited", "Too many tries, wait a few minutes."),
        (
            "unavailable",
            "Something broke on our side, try again later.",
        ),
    ] {
        assert!(
            form.contains(&format!("data-message-{answer}=\"{message}\"")),
            "the {answer} message"
        );
    }
}

#[test]
fn the_newsletter_is_called_eds_daily() {
    for path in PAGES {
        let html = site().file(&format!("{path}index.html"));
        assert!(!html.contains("Vesara Daily"), "{path}");
    }
    assert!(!site().file("index.html").contains("Vesara Daily"));
}
