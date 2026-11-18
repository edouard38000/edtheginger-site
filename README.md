# edtheginger-site

Personal website of edouardtheginger: the Real or BS list, the Playbooks and the Videos.
Served at <https://edtheginger.com/>.

Static site, generated with [Zola](https://www.getzola.org/). Templates and CSS, plus one small self-hosted
script on the Videos page (`static/js/videos.js`, for the Desktop layout; the page works without it)
and one on the Playbook pages (`static/js/playbook.js`, for the Copy buttons; hidden without it):
no third-party scripts, fonts self-hosted. The words used here are defined in [`GLOSSARY.md`](GLOSSARY.md).

## Build and check

You need Rust (cargo, clippy, rustfmt), Zola 0.23.6, [just](https://just.systems/)
(`brew install zola just`) and Google Chrome or Chromium (or set `CHROME` to its path). CI pins the same Zola version in `.github/workflows/site.yml`.

```sh
just check
```

That one command checks formatting and lints, runs the Rust test suite (which builds the site from
the fixture Entries and asserts on the generated pages), checks internal links and builds the real
site into `public/`. One check, `tests/phone_layout.rs`, opens every real Playbook page in headless
Chrome as a 375 px phone and fails on sideways scrolling, anything wider than the screen, reading
text under 16 px, squeezed phone screenshots or buttons under 44 px tall: most readers open the
Playbooks on a phone. `just serve` previews the real site locally.

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
| `templates/playbook.html`, `templates/components/playbook-page.html` | A Playbook page, and the components its Markdown is written with |
| `static/playbooks/<name>/`, `static/playbooks/stickers/` | Screenshots of a Playbook page; the stickers the Playbook pages share |
| `sass/_tokens.scss`, `_fonts.scss`, `_base.scss`, `_dock.scss` | Shared styles; brand tokens are CSS custom properties |
| `sass/pages/_{home,real-or-bs,playbooks,videos}.scss` | One stylesheet partial per page |
| `sass/pages/_playbook.scss` | The Playbook pages |
| `src/lib.rs` | The check harness: fixture Entries in, built pages out |
| `tests/skeleton.rs` | Checks on what every page shares |
| `tests/phone_layout.rs` | The real Playbook pages, measured in headless Chrome on a 375 px phone |
| `tests/{home,real_or_bs,playbooks,videos}.rs` | One test file per page |
| `tests/fixtures/` | Fixture Entries and covers; the checks on the pages never build the real Entries. The fixture Playbook page (`tests/fixtures/playbook-pages/`) is added only by the checks that use it |
| `tests/adding_content.rs`, `tests/bad-entries/` | Checks that a broken Entry is refused, with one bad Entry per refusal; one check reads the real Entries for a missing date |
| `templates/components/validate.html` | The refusals; `templates/entry.html` and every page that lists Entries use them |
| `docs/adding-content.md` | The guide to adding a Video or a Playbook |
| `docs/domain.md` | The DNS records for edtheginger.com and the switch-over steps |
| `static/CNAME`, `tests/domain.rs` | The custom domain, and the checks that it matches `base_url` |
| `brand-assets/` | Source files for the look. Read and copy only |

A change to one page touches its template, its stylesheet partial and its test file.

Every internal link and asset address comes from `base_url`: use `get_url(path="@/videos/_index.md")`
for pages and `get_url(path="covers/x.png")` for static files in templates, and addresses relative
to `main.css` in styles. Never write a link starting with `/`.

## Entries

An Entry is one Markdown file with TOML front matter: a Video in `content/videos/<name>.md`, a
Playbook in `content/playbooks/<name>.md`. A Video has no body. A Playbook page has its whole
Playbook as the body, in Markdown with the components of `templates/components/playbook-page.html`;
a PDF-only Playbook has no body and a `pdf` address. Adding the file (and its cover in
`static/covers/`, and a Playbook page's screenshots in `static/playbooks/<name>/`) is the only step
needed.

[`docs/adding-content.md`](docs/adding-content.md) is the guide: copy-paste templates, naming, the
cover, and what each refusal message means. A broken Entry is refused by `just check` and by CI, so
it never deploys. The refusals are in `templates/components/validate.html` (every Entry is rendered
through `templates/entry.html`, and a page that reads the Entries starts with
`{{<validate.entries formats={config.extra.formats} />}}` so the refusal is printed whichever page
Zola renders first), plus one Rust check for a missing date in `src/lib.rs`; their
tests are `tests/adding_content.rs` with one bad Entry each in `tests/bad-entries/`.

A Playbook with a body is a page of its own, at `/playbooks/<name>/` (`templates/playbook.html`), and
its cards link to it. Every other Entry has no page of its own: Videos link out to TikTok and
Instagram, PDF-only Playbooks to their PDF. Zola still writes an address per Entry
(`/videos/<name>/`), which sends the visitor to the page that lists it.

## Deploy

`.github/workflows/site.yml` runs `just check` on every pull request and every push to `main`, and
on `main` deploys `public/` to GitHub Pages. In the repo settings, Pages must have
"GitHub Actions" as its source.

The site is served at <https://edtheginger.com/>. Three things hold the address: `base_url` in
`config.toml` (every link is built from it), `static/CNAME` (kept in step with it by
`tests/domain.rs`) and the custom domain set in the repo's Pages settings. With a GitHub Actions
deploy, the Pages settings are what actually serve the domain; GitHub ignores the CNAME file. The
DNS records at Porkbun and the switch-over order are in [`docs/domain.md`](docs/domain.md).
