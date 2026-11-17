//! The Real or BS page. Owned by ticket #3: add its checks here.

use site_checks::Site;

#[test]
fn the_real_or_bs_page_has_its_heading() {
    let site = Site::build();
    assert_eq!(site.page("real-or-bs/").texts("main h1"), ["Real or BS"]);
}
