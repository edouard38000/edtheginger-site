//! The home page. Owned by ticket #6: add its checks here.

use site_checks::Site;

#[test]
fn the_home_page_has_its_heading() {
    let site = Site::build();
    assert_eq!(site.page("").texts("main h1"), ["Edouard the Ginger"]);
}
