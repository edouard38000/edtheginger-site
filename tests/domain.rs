//! The custom domain: the site is served over HTTPS at the root of edtheginger.com, and the
//! CNAME file names the same host as `base_url`. That links follow `base_url` at the root is
//! checked in `skeleton.rs` (`internal_links_follow_the_base_url`).

use site_checks::Site;

const ROOT: &str = env!("CARGO_MANIFEST_DIR");
const DOMAIN: &str = "edtheginger.com";

/// The `base_url` value of the real `config.toml`.
fn configured_base_url() -> String {
    let config = std::fs::read_to_string(format!("{ROOT}/config.toml")).unwrap();
    let line = config
        .lines()
        .find(|line| line.trim_start().starts_with("base_url"))
        .expect("config.toml sets base_url");
    let value = line.split_once('=').expect("base_url = \"...\"").1;
    value.trim().trim_matches('"').to_owned()
}

#[test]
fn the_site_is_served_over_https_at_the_root_of_the_domain() {
    assert_eq!(configured_base_url(), format!("https://{DOMAIN}"));
}

#[test]
fn the_cname_file_names_the_host_of_the_base_url() {
    let cname = std::fs::read_to_string(format!("{ROOT}/static/CNAME")).unwrap();
    let host = configured_base_url()
        .strip_prefix("https://")
        .expect("an https base_url")
        .to_owned();
    assert_eq!(cname.trim(), host);
    assert_eq!(cname.lines().count(), 1, "one host, one line");
}

#[test]
fn the_cname_file_is_published_at_the_site_root() {
    assert!(Site::build().has_file("CNAME"));
}
