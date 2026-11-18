//! Checks for edtheginger-site.
//!
//! One seam: Entries in, built pages out. [`Site::build`] puts the real templates, styles,
//! static files and page stubs next to the fixture Entries from `tests/fixtures`, runs
//! `zola build` on the result and gives the generated pages back. The real Entries in
//! `content/` are never built here, so a check on the pages never depends on real content.
//! The checks that read them are [`entries_without_a_date`] and [`clips_refused`].

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use scraper::{Html, Selector};
use tempfile::TempDir;

/// The address the fixture site is built for by default: a sub-path, like the GitHub Pages one.
pub const SUB_PATH_BASE_URL: &str = "https://example.github.io/fixture-site";

/// The four pages of the site, as paths under the base URL.
pub const PAGES: [&str; 4] = ["", "real-or-bs/", "playbooks/", "videos/"];

/// Where the Dock links are on a page.
pub const DOCK_SELECTOR: &str = "nav.dock a";

/// A site built from the fixture Entries.
pub struct Site {
    dir: TempDir,
    base_url: String,
}

impl Site {
    /// Builds the fixture site for [`SUB_PATH_BASE_URL`].
    pub fn build() -> Site {
        Site::build_at(SUB_PATH_BASE_URL)
    }

    /// Builds the fixture site for the given base URL (no trailing slash).
    pub fn build_at(base_url: &str) -> Site {
        let dir = tempfile::tempdir().expect("create a temporary directory");
        assemble(repo_root(), dir.path());
        run_zola(dir.path(), base_url).unwrap_or_else(|message| panic!("{message}"));
        Site {
            dir,
            base_url: base_url.to_owned(),
        }
    }

    /// Builds the fixture site plus extra files, each given as a path inside the Zola project
    /// (e.g. `content/videos/new.md`, `static/covers/new.png`) and its bytes. Unlike
    /// [`Site::build`] it does not panic when the build is refused: the error is what Zola
    /// printed, so a check can read the refusal message of a bad Entry.
    pub fn try_build_with(extra_files: &[(&str, &[u8])]) -> Result<Site, String> {
        Site::try_build_changed(&[], extra_files)
    }

    /// Like [`Site::try_build_with`], but first takes files out of the fixture site, each given
    /// as a path inside the Zola project (e.g. `content/playbooks/prompt-pack.md`). A file of
    /// `extra_files` with the same path as a fixture replaces it.
    pub fn try_build_changed(
        removed_files: &[&str],
        extra_files: &[(&str, &[u8])],
    ) -> Result<Site, String> {
        let dir = tempfile::tempdir().expect("create a temporary directory");
        assemble(repo_root(), dir.path());
        for path in removed_files {
            let target = dir.path().join(path);
            fs::remove_file(&target).unwrap_or_else(|e| panic!("remove {}: {e}", target.display()));
        }
        for (path, bytes) in extra_files {
            let target = dir.path().join(path);
            let folder = target.parent().expect("a file inside the site");
            fs::create_dir_all(folder)
                .unwrap_or_else(|e| panic!("create {}: {e}", folder.display()));
            fs::write(&target, bytes).unwrap_or_else(|e| panic!("write {}: {e}", target.display()));
        }
        let mut refused = entries_without_a_date(&dir.path().join("content"));
        refused.extend(clips_refused(dir.path()));
        if !refused.is_empty() {
            return Err(refused.join("\n"));
        }
        run_zola(dir.path(), SUB_PATH_BASE_URL)?;
        Ok(Site {
            dir,
            base_url: SUB_PATH_BASE_URL.to_owned(),
        })
    }

    /// The base URL this site was built for.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// The folder holding the generated site.
    pub fn output(&self) -> PathBuf {
        self.dir.path().join("public")
    }

    /// Whether a generated file exists, e.g. `fonts/Montserrat-Bold.ttf`.
    pub fn has_file(&self, path: &str) -> bool {
        self.output().join(path).is_file()
    }

    /// The text of a generated file, e.g. `main.css`.
    pub fn file(&self, path: &str) -> String {
        let full = self.output().join(path);
        fs::read_to_string(&full).unwrap_or_else(|e| panic!("read {}: {e}", full.display()))
    }

    /// The generated page at a path under the base URL: `""` is home, `"videos/"` the Videos page.
    pub fn page(&self, path: &str) -> Page {
        let file = format!("{path}index.html");
        Page {
            path: path.to_owned(),
            html: Html::parse_document(&self.file(&file)),
        }
    }

    /// Every generated HTML page, as paths relative to the output folder.
    pub fn html_files(&self) -> Vec<String> {
        let mut found = Vec::new();
        collect_html(&self.output(), &self.output(), &mut found);
        found.sort();
        found
    }
}

/// One generated page.
pub struct Page {
    path: String,
    html: Html,
}

impl Page {
    /// Parses a generated HTML file of the site.
    pub fn from_file(site: &Site, file: &str) -> Page {
        Page {
            path: file.to_owned(),
            html: Html::parse_document(&site.file(file)),
        }
    }

    /// The page's path, for messages.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// The text of every element matching a CSS selector, trimmed.
    pub fn texts(&self, selector: &str) -> Vec<String> {
        let selector = parse_selector(selector);
        self.html
            .select(&selector)
            .map(|element| element.text().collect::<String>().trim().to_owned())
            .collect()
    }

    /// The value of an attribute on every element matching a CSS selector that has it.
    pub fn attributes(&self, selector: &str, attribute: &str) -> Vec<String> {
        let selector = parse_selector(selector);
        self.html
            .select(&selector)
            .filter_map(|element| element.value().attr(attribute))
            .map(str::to_owned)
            .collect()
    }

    /// How many elements match a CSS selector.
    pub fn count(&self, selector: &str) -> usize {
        let selector = parse_selector(selector);
        self.html.select(&selector).count()
    }

    /// The Dock links on this page, as (address, accessible label) pairs, in order.
    pub fn dock_links(&self) -> Vec<(String, String)> {
        let selector = parse_selector(DOCK_SELECTOR);
        self.html
            .select(&selector)
            .map(|link| {
                (
                    link.value().attr("href").unwrap_or_default().to_owned(),
                    link.value()
                        .attr("aria-label")
                        .unwrap_or_default()
                        .to_owned(),
                )
            })
            .collect()
    }
}

/// Refuses every Entry under a `content` folder that has no `date`, with one message each.
///
/// This one refusal is Rust and not a template check because Zola never renders such an Entry:
/// the Videos and Playbooks are sorted by date, so Zola leaves an Entry with no date out of the
/// site with only a warning, and `templates/entry.html` never sees it.
pub fn entries_without_a_date(content: &Path) -> Vec<String> {
    let mut files = Vec::new();
    for section in ["videos", "playbooks"] {
        collect_entries(content, &content.join(section), &mut files);
    }
    files.sort();
    files
        .iter()
        .filter(|file| {
            let text =
                fs::read_to_string(file).unwrap_or_else(|e| panic!("read {}: {e}", file.display()));
            !front_matter_has_date(&text)
        })
        .map(|file| {
            let entry = file.strip_prefix(content).expect("a file inside content");
            format!(
                "Entry content/{} refused: the Entry has no date: set `date` above `[extra]`, \
                 like 2026-11-08, without quotes. See docs/adding-content.md.",
                entry.to_string_lossy().replace('\\', "/")
            )
        })
        .collect()
}

/// The most a Clip may weigh, in bytes (400 KB). The Clip command aims well under it.
pub const CLIP_BUDGET_BYTES: u64 = 400_000;

/// Refuses every Video Entry of a Zola project (`site` holds `content/` and `static/`) whose
/// Clip file is missing or over [`CLIP_BUDGET_BYTES`], with one message each.
///
/// These two refusals are Rust and not template checks because Zola can neither read the size
/// of an MP4 nor tell whether one exists. An Entry with no `clip`, or one that is not a path in
/// `static/clips/`, is left to `templates/components/validate.html`.
pub fn clips_refused(site: &Path) -> Vec<String> {
    let content = site.join("content");
    let mut files = Vec::new();
    collect_entries(&content, &content.join("videos"), &mut files);
    files.sort();
    let mut refused = Vec::new();
    for file in files {
        let text =
            fs::read_to_string(&file).unwrap_or_else(|e| panic!("read {}: {e}", file.display()));
        let Some(clip) = front_matter_string(&text, "clip") else {
            continue;
        };
        if !clip.starts_with("clips/") || !clip.ends_with(".mp4") || clip.contains("..") {
            continue;
        }
        let entry = file.strip_prefix(&content).expect("a file inside content");
        let entry = entry.to_string_lossy().replace('\\', "/");
        let problem = match fs::metadata(site.join("static").join(&clip)) {
            Err(_) => format!(
                "the Clip file static/{clip} does not exist: make it with the Clip command."
            ),
            Ok(meta) if meta.len() > CLIP_BUDGET_BYTES => format!(
                "the Clip static/{clip} is {} bytes, over the budget of {CLIP_BUDGET_BYTES} \
                 bytes (400 KB): make it again with the Clip command.",
                meta.len()
            ),
            Ok(_) => continue,
        };
        refused.push(format!(
            "Entry content/{entry} refused: {problem} See docs/adding-content.md."
        ));
    }
    refused
}

/// The text of a `key = "value"` line in the front matter (between the two `+++` lines).
fn front_matter_string(text: &str, key: &str) -> Option<String> {
    text.lines()
        .skip_while(|line| line.trim() != "+++")
        .skip(1)
        .map(str::trim)
        .take_while(|line| *line != "+++")
        .find_map(|line| {
            let rest = line.strip_prefix(key)?.trim_start().strip_prefix('=')?;
            let value = rest.trim().strip_prefix('"')?.strip_suffix('"')?;
            Some(value.to_owned())
        })
}

fn collect_entries(content: &Path, dir: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries {
        let path = entry.expect("read a directory entry").path();
        if path.is_dir() {
            collect_entries(content, &path, found);
        } else if is_entry(content, &path) && path.extension().is_some_and(|e| e == "md") {
            found.push(path);
        }
    }
}

/// Whether the TOML front matter (between the two `+++` lines) sets a top-level `date`.
fn front_matter_has_date(text: &str) -> bool {
    text.lines()
        .skip_while(|line| line.trim() != "+++")
        .skip(1)
        .map(str::trim)
        .take_while(|line| *line != "+++" && !line.starts_with('['))
        .any(|line| {
            line.strip_prefix("date")
                .is_some_and(|rest| rest.trim_start().starts_with('='))
        })
}

fn parse_selector(selector: &str) -> Selector {
    Selector::parse(selector).unwrap_or_else(|e| panic!("bad selector {selector:?}: {e}"))
}

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

/// Lays out a Zola project in `site`: the real skeleton plus the fixture Entries.
fn assemble(root: &Path, site: &Path) {
    fs::copy(root.join("config.toml"), site.join("config.toml")).expect("copy config.toml");
    copy_tree(&root.join("templates"), &site.join("templates"), &|_| true);
    copy_tree(&root.join("sass"), &site.join("sass"), &|_| true);
    // Static files without the real covers and Clips.
    let real_covers = root.join("static").join("covers");
    let real_clips = root.join("static").join("clips");
    copy_tree(&root.join("static"), &site.join("static"), &|path| {
        !path.starts_with(&real_covers) && !path.starts_with(&real_clips)
    });
    // The page stubs and section files, without the real Entries.
    copy_tree(&root.join("content"), &site.join("content"), &|path| {
        !is_entry(&root.join("content"), path)
    });

    let fixtures = root.join("tests").join("fixtures");
    for section in ["videos", "playbooks"] {
        copy_tree(
            &fixtures.join(section),
            &site.join("content").join(section),
            &|_| true,
        );
    }
    copy_tree(
        &fixtures.join("covers"),
        &site.join("static").join("covers"),
        &|_| true,
    );
    copy_tree(
        &fixtures.join("clips"),
        &site.join("static").join("clips"),
        &|_| true,
    );
}

/// An Entry is a content file in `videos/` or `playbooks/` other than the section's own `_index.md`.
fn is_entry(content: &Path, path: &Path) -> bool {
    let in_entry_folder = ["videos", "playbooks"]
        .iter()
        .any(|section| path.starts_with(content.join(section)));
    in_entry_folder
        && path.is_file()
        && path.file_name().and_then(|name| name.to_str()) != Some("_index.md")
}

fn copy_tree(from: &Path, to: &Path, keep: &dyn Fn(&Path) -> bool) {
    fs::create_dir_all(to).unwrap_or_else(|e| panic!("create {}: {e}", to.display()));
    let entries = fs::read_dir(from).unwrap_or_else(|e| panic!("read {}: {e}", from.display()));
    for entry in entries {
        let path = entry.expect("read a directory entry").path();
        if !keep(&path) {
            continue;
        }
        let target = to.join(path.file_name().expect("a file name"));
        if path.is_dir() {
            copy_tree(&path, &target, keep);
        } else {
            fs::copy(&path, &target).unwrap_or_else(|e| panic!("copy {}: {e}", path.display()));
        }
    }
}

fn collect_html(root: &Path, dir: &Path, found: &mut Vec<String>) {
    for entry in fs::read_dir(dir).unwrap_or_else(|e| panic!("read {}: {e}", dir.display())) {
        let path = entry.expect("read a directory entry").path();
        if path.is_dir() {
            collect_html(root, &path, found);
        } else if path.extension().and_then(|e| e.to_str()) == Some("html") {
            let relative = path.strip_prefix(root).expect("a path inside the output");
            found.push(relative.to_string_lossy().replace('\\', "/"));
        }
    }
}

/// Runs `zola build` in `site`. The binary is `zola` on the PATH, or the one named by `ZOLA`.
fn run_zola(site: &Path, base_url: &str) -> Result<(), String> {
    let zola = std::env::var("ZOLA").unwrap_or_else(|_| "zola".to_owned());
    let output = Command::new(&zola)
        .current_dir(site)
        .args(["build", "--base-url", base_url])
        .output()
        .map_err(|e| format!("could not run `{zola}` (is Zola installed and on the PATH?): {e}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "zola build failed:\n{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
