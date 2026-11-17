//! The Playbooks page. Owned by ticket #4: add its checks here.

use site_checks::Site;

#[test]
fn the_playbooks_page_has_its_heading() {
    let site = Site::build();
    assert_eq!(site.page("playbooks/").texts("main h1"), ["Playbooks"]);
}
