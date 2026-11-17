# edtheginger-site

Personal website of edouardtheginger: the Real or BS list, the Playbooks and the Videos.
Served first at <https://edouard38000.github.io/edtheginger-site/>, later at edtheginger.com.

Static site, generated with [Zola](https://www.getzola.org/). Templates and CSS only: no JavaScript,
no third-party scripts, fonts self-hosted. The words used here are defined in [`GLOSSARY.md`](GLOSSARY.md).

## Build and check

You need Rust (cargo, clippy, rustfmt), Zola 0.23.6 and [just](https://just.systems/)
(`brew install zola just`). CI pins the same Zola version in `.github/workflows/site.yml`.

```sh
just check
```

That one command checks formatting and lints, runs the Rust test suite (which builds the site from
the fixture Entries and asserts on the generated pages), checks internal links and builds the real
site into `public/`. `just serve` previews the real site locally.

The task runner is a `justfile` and not Rust code because it only chains four commands; `just`
itself is a Rust tool.

## Layout

| Path | What it holds |
| --- | --- |
| `config.toml` | Site settings: the one `base_url`, the Formats, the Dock links (`[extra.dock]`) |
| `content/videos/*.md`, `content/playbooks/*.md` | The real Entries |
| `content/_index.md`, `content/real-or-bs.md`, `content/*/_index.md` | The four pages (not Entries) |
| `static/covers/` | Cover images of the real Entries |
| `static/fonts/` | Montserrat, copied from `brand-assets/fonts/` |
| `templates/base.html`, `templates/partials/dock.html` | Base layout and the Dock, shared by every page |
| `templates/{index,real-or-bs,playbooks,videos}.html` | One template per page |
| `sass/_tokens.scss`, `_fonts.scss`, `_base.scss`, `_dock.scss` | Shared styles; brand tokens are CSS custom properties |
| `sass/pages/_{home,real-or-bs,playbooks,videos}.scss` | One stylesheet partial per page |
| `src/lib.rs` | The check harness: fixture Entries in, built pages out |
| `tests/skeleton.rs` | Checks on what every page shares |
| `tests/{home,real_or_bs,playbooks,videos}.rs` | One test file per page |
| `tests/fixtures/` | Fixture Entries and covers; the checks never read the real Entries |
| `brand-assets/` | Source files for the look. Read and copy only |

A change to one page touches its template, its stylesheet partial and its test file.

Every internal link and asset address comes from `base_url`: use `get_url(path="@/videos/_index.md")`
for pages and `get_url(path="covers/x.png")` for static files in templates, and addresses relative
to `main.css` in styles. Never write a link starting with `/`.

## Entries

An Entry is one Markdown file with TOML front matter and no body: a Video in
`content/videos/<name>.md`, a Playbook in `content/playbooks/<name>.md`. Adding the file (and its
cover in `static/covers/`) is the only step needed.

[`docs/adding-content.md`](docs/adding-content.md) is the guide: copy-paste templates, naming, the
cover, and what each refusal message means. A broken Entry is refused by `just check` and by CI, so
it never deploys. The refusals are in `templates/components/validate.html` (every Entry is rendered
through `templates/entry.html`), plus one Rust check for a missing date in `src/lib.rs`; their
tests are `tests/adding_content.rs` with one bad Entry each in `tests/bad-entries/`.

An Entry has no page of its own: Videos link out to TikTok and Instagram, Playbooks to their PDF.
Zola still writes an address per Entry (`/videos/<name>/`), which sends the visitor to the page
that lists it.

## Deploy

`.github/workflows/site.yml` runs `just check` on every pull request and every push to `main`, and
on `main` deploys `public/` to GitHub Pages. In the repo settings, Pages must have
"GitHub Actions" as its source.

To move to edtheginger.com, change `base_url` in `config.toml`; nothing else holds the address.
