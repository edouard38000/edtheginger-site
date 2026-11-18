//! Phone first: every Playbook page, read on a 375 px wide phone.
//!
//! Almost every reader opens a Playbook page on a phone, from an Instagram DM or a TikTok bio
//! link. This check builds the real site (the real Playbook pages, not the fixtures: the rule is
//! about what readers get), serves it on a local port and opens each Playbook page in headless
//! Chrome as a 375x812 phone. It fails when a page:
//!
//! - scrolls sideways, or has anything wider than the screen;
//! - has reading text under 16 px (the short labels and buttons in orange boxes may go down to
//!   13 px: they are tags, not text to read);
//! - shows a phone screenshot under 240 px wide (too small to read what is on it);
//! - has a button or link button under 44 px tall.
//!
//! It needs Google Chrome or Chromium: found on the PATH, in /Applications on a Mac, or named by
//! the `CHROME` environment variable. GitHub's Ubuntu runners have it.

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;

use headless_chrome::protocol::cdp::Emulation;
use headless_chrome::{Browser, LaunchOptions};

/// The phone the pages are checked on: an iPhone 13 mini / SE-class screen.
const PHONE_WIDTH: u32 = 375;
const PHONE_HEIGHT: u32 = 812;

/// Runs in the page. Returns one line per problem found, joined with newlines.
const PROBE: &str = r#"(async () => {
  await document.fonts.ready;
  const problems = [];
  const doc = document.documentElement;
  const width = doc.clientWidth;
  const name = (el) => el.tagName.toLowerCase() + (el.className && typeof el.className === "string"
    ? "." + el.className.trim().split(/\s+/).join(".") : "");

  // Sideways scrolling, and anything that sticks out of the screen.
  if (doc.scrollWidth > width) {
    problems.push(`scrolls sideways: the page is ${doc.scrollWidth} px wide on a ${width} px screen`);
  }
  for (const el of document.body.querySelectorAll("*")) {
    if (el.closest(".pb-sr-only")) continue;
    const box = el.getBoundingClientRect();
    if (box.width > 0 && (box.right > width + 0.5 || box.left < -0.5)) {
      problems.push(`wider than the screen: ${name(el)} spans ${Math.round(box.left)} to ${Math.round(box.right)} px`);
    }
  }

  // Text size. Short labels and buttons in orange boxes are tags, allowed down to 13 px.
  const tags = ".pb-label, .pb-step__n, .pb-button, .pb-buttons__click, .pb-prompt__copy";
  const small = new Map();
  const walker = document.createTreeWalker(document.querySelector("article.pb"), NodeFilter.SHOW_TEXT);
  for (let node = walker.nextNode(); node; node = walker.nextNode()) {
    const el = node.parentElement;
    if (!node.textContent.trim() || el.closest(".pb-sr-only") || !el.getClientRects().length) continue;
    const size = parseFloat(getComputedStyle(el).fontSize);
    const floor = el.closest(tags) ? 13 : 16;
    if (size < floor) {
      const key = `text under ${floor} px: ${name(el)} at ${size} px`;
      small.set(key, (small.get(key) || [])
        .concat(node.textContent.trim().slice(0, 40)));
    }
  }
  for (const [key, texts] of small) problems.push(`${key} (${texts.length}x, e.g. "${texts[0]}")`);

  // Phone screenshots (taller than wide) wide enough to read what is on them.
  for (const img of document.querySelectorAll(".pb img.pb-shot__image, .pb img.pb-figure__image")) {
    const tall = img.height && img.getAttribute("height") > img.getAttribute("width") * 1.3;
    const shown = img.getBoundingClientRect().width;
    if (tall && shown > 0 && shown < 240) {
      problems.push(`phone screenshot under 240 px wide: ${img.alt.slice(0, 40) || img.src} is ${Math.round(shown)} px`);
    }
  }

  // Tap targets.
  for (const el of document.querySelectorAll(".pb a.pb-button, .pb button, .pb__back, .pb-skill__grab")) {
    const box = el.getBoundingClientRect();
    if (box.height > 0 && box.height < 44) {
      problems.push(`tap target under 44 px tall: ${name(el)} "${el.textContent.trim().slice(0, 30)}" is ${Math.round(box.height)} px`);
    }
  }
  return problems.join("\n");
})()"#;

#[test]
fn every_playbook_page_reads_well_on_a_375_px_phone() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("open a local port");
    let base_url = format!("http://{}", listener.local_addr().expect("local address"));
    let out = tempfile::tempdir().expect("create a temporary directory");
    let public = out.path().join("public");
    build_real_site(&base_url, &public);
    serve(listener, public.clone());

    let pages = playbook_pages(&public);
    assert!(
        !pages.is_empty(),
        "no Playbook page found under playbooks/ in the built site"
    );

    let browser = Browser::new(
        LaunchOptions::default_builder()
            .window_size(Some((PHONE_WIDTH, PHONE_HEIGHT)))
            // Only our own pages are opened. CI runners block Chrome's sandbox.
            .sandbox(false)
            .build()
            .expect("browser options"),
    )
    .unwrap_or_else(|e| {
        panic!("could not start Chrome (install Google Chrome or Chromium, or set CHROME): {e}")
    });

    let mut failures = Vec::new();
    for page in &pages {
        let url = format!("{base_url}/playbooks/{page}/");
        let tab = browser.new_tab().expect("open a tab");
        tab.call_method(Emulation::SetDeviceMetricsOverride {
            width: PHONE_WIDTH,
            height: PHONE_HEIGHT,
            device_scale_factor: 2.0,
            mobile: true,
            scale: None,
            screen_width: None,
            screen_height: None,
            position_x: None,
            position_y: None,
            dont_set_visible_size: None,
            screen_orientation: None,
            viewport: None,
            display_feature: None,
            device_posture: None,
        })
        .expect("emulate a phone screen");
        tab.navigate_to(&url)
            .and_then(|tab| tab.wait_until_navigated())
            .unwrap_or_else(|e| panic!("open {url}: {e}"));
        let result = tab
            .evaluate(PROBE, true)
            .unwrap_or_else(|e| panic!("measure {url}: {e}"));
        let found = result
            .value
            .as_ref()
            .and_then(|value| value.as_str())
            .unwrap_or_else(|| panic!("measure {url}: no result"))
            .to_owned();
        for problem in found.lines() {
            failures.push(format!("playbooks/{page}/: {problem}"));
        }
        let _ = tab.close(true);
    }

    assert!(
        failures.is_empty(),
        "Playbook pages that do not read well on a {PHONE_WIDTH} px phone:\n{}",
        failures.join("\n")
    );
}

/// Builds the real site, Entries included, for `base_url` into `public`.
fn build_real_site(base_url: &str, public: &Path) {
    let zola = std::env::var("ZOLA").unwrap_or_else(|_| "zola".to_owned());
    let output = Command::new(&zola)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["build", "--base-url", base_url, "--output-dir"])
        .arg(public)
        .output()
        .unwrap_or_else(|e| panic!("could not run `{zola}`: {e}"));
    assert!(
        output.status.success(),
        "zola build failed:\n{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

/// The Playbook pages of the built site: every `playbooks/<name>/index.html` holding a Playbook
/// page (a PDF-only Playbook writes a redirect there instead).
fn playbook_pages(public: &Path) -> Vec<String> {
    let mut pages: Vec<String> = fs::read_dir(public.join("playbooks"))
        .expect("read playbooks/ of the built site")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            let html = fs::read_to_string(path.join("index.html")).ok()?;
            html.contains("<article class=\"pb\"")
                .then(|| path.file_name()?.to_str().map(str::to_owned))?
        })
        .collect();
    pages.sort();
    pages
}

/// Serves `root` over HTTP on `listener`, in the background, for as long as the test runs.
fn serve(listener: TcpListener, root: PathBuf) {
    thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let root = root.clone();
            thread::spawn(move || answer(stream, &root));
        }
    });
}

fn answer(mut stream: TcpStream, root: &Path) {
    let mut request_line = String::new();
    let mut reader = BufReader::new(&stream);
    if reader.read_line(&mut request_line).is_err() {
        return;
    }
    // Skip the headers.
    let mut line = String::new();
    while reader.read_line(&mut line).map(|n| n > 2).unwrap_or(false) {
        line.clear();
    }
    let path = request_line.split_whitespace().nth(1).unwrap_or("/");
    let path = path.split(['?', '#']).next().unwrap_or("/");
    let mut file = root.join(path.trim_start_matches('/'));
    if path.split('/').any(|part| part == "..") {
        file = root.join("404.html");
    } else if file.is_dir() {
        file = file.join("index.html");
    }
    let (status, body) = match fs::read(&file) {
        Ok(body) => ("200 OK", body),
        Err(_) => ("404 Not Found", Vec::new()),
    };
    let header = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        content_type(&file),
        body.len()
    );
    let _ = stream.write_all(header.as_bytes());
    let _ = stream.write_all(&body);
}

fn content_type(file: &Path) -> &'static str {
    match file.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css",
        "js" => "text/javascript",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "ttf" => "font/ttf",
        "woff2" => "font/woff2",
        _ => "application/octet-stream",
    }
}
