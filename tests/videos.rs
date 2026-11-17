//! The Videos page. Owned by ticket #5: add its checks here.

use site_checks::Site;

#[test]
fn the_videos_page_has_its_heading() {
    let site = Site::build();
    assert_eq!(site.page("videos/").texts("main h1"), ["Videos"]);
}
