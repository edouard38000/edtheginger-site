# Adding content

How to put a new Video or Playbook on the site. Written for an editing session: follow it at the
end of an edit, without asking. The words used here are defined in [`GLOSSARY.md`](../GLOSSARY.md).

One Video or one Playbook is one Entry: a Markdown file with TOML front matter between two `+++`
lines. A Video and a PDF-only Playbook have nothing after them; a Playbook page has its whole
Playbook written after them, in Markdown (see [Template 4](#template-4-a-playbook-page)). Adding the
Entry and its cover is the whole job. The Tally, the
numbering of the REAL list and the latest Video on the home page are computed from the Entries at
build time. Never type them anywhere.

## Steps

1. Put the cover in `static/covers/`.
2. For a Video, make its Clip (see [Add the Clip](#add-the-clip)).
3. Add the Entry: copy one of the four templates below and change the values.
4. Run `just check`. If it refuses the Entry, fix what the message says and run it again.
5. Commit and push straight to `main` (see [Push](#push)).

## The cover

- Where: `static/covers/`. In the Entry, write the path without `static/`: `cover = "covers/<file>"`.
- Shape: portrait 9:16, PNG or JPEG (`.png`, `.jpg` or `.jpeg`), 1080 x 1920 if you have it and
  never smaller than 540 x 960. The site resizes it; keep the original.
- Name: the same name as the Entry file. The Video `content/videos/real-or-bs-02-phone-use.md` has
  the cover `static/covers/real-or-bs-02-phone-use.png`. A Playbook cover ends in `-playbook`:
  `static/covers/phone-use-playbook.png`.
- A Video must have a cover. A Playbook may have none.
- No cover ready yet: use `cover = "covers/placeholder-cover.png"` and replace it later.

## Add the Clip

Every Video has a Clip: a silent five-second loop of its hook, hosted on the site and shown on the
home page's Wheel. The Clip command makes it, and its still, from the Video's file (the exported
video, or the posted iteration with the most TikTok views). Run it from the repo root with the
Entry's file name without `.md`:

```sh
cargo run --release --manifest-path scripts/clip/Cargo.toml -- ~/Movies/my-video.mp4 real-or-bs-02-phone-use
```

It needs ffmpeg (`brew install ffmpeg`). It takes the first five seconds, crops them to 360 x 640,
plays them at 60 fps (Chrome draws the page at the frame rate of its videos, so the Wheel turns at
60 fps), drops the sound and writes `static/clips/real-or-bs-02-phone-use.mp4` (H.264, fast-start,
well under the 400 KB budget) and its still `static/clips/real-or-bs-02-phone-use.webp`, then prints the
two lines to put under `[extra]`:

```
clip = "clips/real-or-bs-02-phone-use.mp4"
still = "clips/real-or-bs-02-phone-use.webp"
```

Never put the source video in the repo. Only the Clip and the still go in `static/clips/`.

## Naming an Entry

Lowercase letters, digits and hyphens, ending in `.md`. The name becomes an address on the site, so
do not rename an Entry once it is pushed.

- Real or BS Video: `content/videos/real-or-bs-<episode, two digits>-<subject>.md`,
  like `real-or-bs-02-phone-use.md`.
- Any other Video: `content/videos/<format>-<subject>.md`, like `github-tools-repo-radar.md`,
  `storytelling-first-hackathon.md` or `talking-head-ai-jobs.md`.
- Playbook: `content/playbooks/<subject>.md`, like `phone-use.md`.

## Template 1: a Video with no Verdict

For every Format except Real or BS. File: `content/videos/github-tools-repo-radar.md`.

```toml
+++
title = "Repo Radar"
date = 2026-11-15
description = "One or two lines about the Video."

[extra]
format = "GitHub tools"
cover = "covers/github-tools-repo-radar.png"
tiktok = "https://www.tiktok.com/@edouardtheginger/video/1234567890123456789"
instagram = "https://www.instagram.com/reel/ABC123xyz/"
clip = "clips/github-tools-repo-radar.mp4"
still = "clips/github-tools-repo-radar.webp"
+++
```

- `date` is the posting date, written `YYYY-MM-DD` without quotes.
- `format` is exactly one of: `Real or BS`, `GitHub tools`, `Storytelling`, `Talking head`.
- `clip` and `still` are the two files the Clip command wrote, in `static/clips/` (see
  [Add the Clip](#add-the-clip)).
- `tiktok` and `instagram` are the full https addresses of the posted Video. When the Video was
  posted as several iterations, link the iteration with the most views. Leave out the line of
  a link you do not have yet and add it later. Never write an empty or made-up link.
- Do not add `verdict`, `episode` or `tested`: they are only for Real or BS.

## Template 2: a Real or BS Video with a Verdict and a Playbook

File: `content/videos/real-or-bs-02-phone-use.md`.

```toml
+++
title = "Phone Use"
date = 2026-11-08
description = "Can Claude really use an iPhone by itself?"

[extra]
format = "Real or BS"
cover = "covers/real-or-bs-02-phone-use.png"
tiktok = "https://www.tiktok.com/@edouardtheginger/video/1234567890123456789"
instagram = "https://www.instagram.com/reel/ABC123xyz/"
episode = 2
tested = "Claude using an iPhone by itself to fill an Uber Eats basket"
verdict = "real"
playbook = "playbooks/phone-use.md"
clip = "clips/real-or-bs-02-phone-use.mp4"
still = "clips/real-or-bs-02-phone-use.webp"
+++
```

- `episode` is the episode number, a whole number without quotes.
- `tested` is one line saying what was tested.
- `verdict` is `"real"` or `"bs"`, lowercase. A Real or BS Video must have one.
- `playbook` is the file of the Playbook Entry, as a path under `content/`. Leave the line out when
  the Video has no Playbook. Add the Playbook Entry (template 3) in the same commit, or before.

## Template 3: a PDF-only Playbook

For a Playbook that is only a PDF on the playbook site. Its card links to the PDF. New Playbooks
are pages: see [Template 4](#template-4-a-playbook-page). File: `content/playbooks/phone-use.md`.

```toml
+++
title = "Phone Use"
date = 2026-11-08
description = "Let Claude use an iPhone by itself, like filling an Uber Eats basket."

[extra]
pdf = "https://edouard38000.github.io/edtheginger-playbook/phone-use-playbook.pdf"
cover = "covers/phone-use-playbook.png"
+++
```

- `description` says what the Playbook helps you do.
- `pdf` is the https address where the PDF already is on the playbook site. The PDF is never copied
  or moved into this repo.
- `cover` is optional: leave the line out when there is none.

## Template 4: a Playbook page

A Playbook page is a page of the site, at `/playbooks/<name>/`, written in Markdown below the front
matter. Its card on the Playbooks page and on the home shelf links to it. Leave `pdf` out unless the
page should also keep an old PDF; the page never links to it. File: `content/playbooks/voice-notes.md`.

```toml
+++
title = "Turn your voice notes into a to-do list"
date = 2026-11-20
description = "Send a voice note, get a to-do list back."

[extra]
cover = "covers/voice-notes-playbook.png"
+++
One short paragraph: what the reader gets at the end.

{{<subhead text="What you need" icon="check" level={2} />}}

{%<row key="A paid Claude plan">%}Claude Code does the work.{%</row>%}

{{<subhead text="The two parts" level={2} />}}

1. [**Record the note.**](#part-1) Ten seconds is enough.
2. [**Let Claude sort it.**](#part-2) One prompt.

{%<part n={1} id="part-1" title="Record the note">%}
The intro of the part.
{%</part>%}

{%<step n={1} title="Open Voice Memos">%}It is already on your iPhone.{%</step>%}

{%<part n={2} id="part-2" title="Let Claude sort it">%}
The intro of the part.
{%</part>%}

{%<prompt label="PROMPT" dest="paste in Claude Code" logos="claude">%}
Turn this voice note into a to-do list for [YOUR WEEK].
{%</prompt>%}
```

The page puts "by edouardtheginger" and the title in the orange box at the top; write everything
else in the body. Plain Markdown works as usual: paragraphs, **bold**, links, and numbered lists
(shown with orange number stickers, used for the list of parts). The blocks of a Playbook are
components, written `{{<name ... />}}`, or `{%<name ...>%}...{%</name>%}` when they have a body.
Text goes in quotes, and numbers and lists go in braces: `n={3}`, `srcs={["a.jpg", "b.jpg"]}`. Use
single quotes around text that holds double quotes: `title='Say "Hey Claude" three times'`.

| Component | What it shows | Arguments |
| --- | --- | --- |
| `part` | A part: its number sticker, its title in the orange box, its intro (the body). | `n` (1 to 8, the sticker), `id` (the anchor the list of parts links to, like `part-3`), `title`; `label=""` for a part that is not numbered, like "The result". |
| `subhead` | A heading with an optional sticker before it. | `text`; `icon` = `"check"`, `"lightbulb"`, `"exclamation"` or `"clock"`; `level={2}` at the top of the page (default 3, inside a part). |
| `step` | A numbered step: "01" in an orange tag, the step in bold, then what to do (the body, Markdown). Without a title, the "01" sits beside the text, like a numbered line of the PDF. | `n`; `title` (optional). |
| `row` | A key in an orange tag and its explanation (the body), like the "What you need" list. | `key`. |
| `label` | A small orange label on its own line, like "THIS ERASES THE WHOLE PC". | `text`. |
| `note` | A small grey note (the body). | none. |
| `prompt` | A prompt box: the label, where to paste it, the exact text (the body, raw text: never Markdown) and a Copy button. Placeholders in square brackets, like `[YOUR CITY]`, are shown in orange. | `label` (default `PROMPT`), `dest` (default `paste in ChatGPT or Claude`), `logos` (`"chatgpt,claude"` by default, `"claude"`, `"chatgpt"` or `""`), `example_label` and `example` for an example under the text (not copied). |
| `skill` | Under a prompt that uses a skill: a plus, SKILL, and a box with the skill's name and GRAB IT HERE. | `name`; `url` only when the skill has a real public address. |
| `step_image` | One screenshot with rounded corners: as wide as the text on phones; on wide screens centred and capped by its shape (a phone screenshot stays narrow, a slide wider). | `src` (a path under `static/`), `alt` (what the image shows), `label`, `caption`. |
| `shots` | Phone screenshots side by side, each under its label. | `srcs`, `alts`, `labels`: lists of the same length. |
| `beside` | Screenshots beside the steps they go with (the body: steps, rows, text). From 60rem wide the screenshots sit on the left of the body; on phones they come first, like `shots`. Best with one or two phone screenshots. | `srcs`, `alts`, `labels`, as for `shots`. |
| `buttons` | Orange pill links, then the crayon arrow and CLICK HERE. | `texts`, `urls`: lists of the same length. |

- The prompt text is shown and copied exactly as written in the body, between the two tags on
  their own lines: blank lines, numbered lines, `<`, `&`, quotes and `[PLACEHOLDERS]` all stay as
  they are. Markdown is not applied to it.
- A step with no title, for a numbered line like "01  Write a short company brief":
  `{%<step n={1}>%}Write a short company brief.{%</step>%}`.
- Screenshots beside their steps:

  ```
  {%<beside srcs={["playbooks/<name>/app.jpg"]} labels={["THE RIGHT APP"]} alts={["What it shows."]}>%}
  {%<step n={1} title="Install the app">%}...{%</step>%}

  {%<step n={2} title="Scan the QR code">%}...{%</step>%}
  {%</beside>%}
  ```
- Screenshots go in `static/playbooks/<name>/`, as JPEG or PNG; the site resizes them. Every image
  needs an `alt` saying what it shows.
- A component prints one block: leave a blank line before and after it.
- The components are in `templates/components/playbook-page.html`, their styles in
  `sass/pages/_playbook.scss`.

## Check locally

```sh
just check
```

It needs Rust, Zola 0.23.6 and just (`brew install zola just`). It runs the test suite and builds
the real site with every Entry in `content/`. Green means the Entry is accepted. The build stops at
the first refused Entry and prints a line like this one among Zola's own lines:

```
Entry content/videos/real-or-bs-02-phone-use.md refused: the Verdict "maybe" is not "real" or "bs". See docs/adding-content.md.
```

Links are checked for their shape only. Nothing is fetched, so open each link yourself once.

## Push

A content-only change touches nothing but `content/videos/`, `content/playbooks/`,
`static/covers/`, `static/clips/` and the screenshots of a Playbook page in `static/playbooks/<name>/`. Commit it and push it straight to `main`, with no pull request:

```sh
git pull --rebase
just check
git add content static/covers static/clips static/playbooks
git commit -m "Add the Video real-or-bs-02-phone-use"
git push
```

Every push to `main` runs the same `just check` on GitHub and, when it is green, deploys the site.
No one has to do anything else. If the check fails there, nothing is deployed and the site stays
as it was: fix the Entry and push again.

Anything else (templates, styles, checks, the workflow, `config.toml`, this guide) goes through a
pull request.

## What each refusal means

Each message starts with `Entry content/<file> refused:` and names the file to fix.

| The message says | What is wrong | Fix |
| --- | --- | --- |
| `the Entry has no title` | `title` is missing or empty. | Set `title` above `[extra]`. |
| `the Entry has no date` | `date` is missing, or it is written under `[extra]`. | Set `date = YYYY-MM-DD` above `[extra]`. |
| `the Video has no Format` | `format` is missing. | Set `format` under `[extra]`. |
| `unknown Format` | `format` is not one of the four Formats, letter for letter. | Use `Real or BS`, `GitHub tools`, `Storytelling` or `Talking head`. |
| `the Video has no cover` | `cover` is missing. | Set `cover`, or use the placeholder cover. |
| `` `cover` must be the path of a PNG or JPEG `` | `cover` starts with `/`, or does not end in `.png`, `.jpg` or `.jpeg`. | Write it like `covers/<file>.png`. |
| `the cover file static/... does not exist` | No file at that path under `static/`. Often a typo, `static/` written in the path, or the image was not added. | Add the image or correct the path. |
| `is not a well-formed https address` | `tiktok`, `instagram` or `pdf` does not start with `https://`, or has a space or a broken host name. | Paste the full address. Leave out a `tiktok` or `instagram` line you have no link for. |
| `a Real or BS Video needs a Verdict` | The Format is Real or BS and `verdict` is missing. | Set `verdict = "real"` or `verdict = "bs"`. |
| `is not "real" or "bs"` | `verdict` is something else, like `"REAL"` or `"fake"`. | Use `"real"` or `"bs"`, lowercase. |
| `` a Real or BS Video needs `episode` `` | `episode` is missing, in quotes, or below 1. | Set `episode = 3`, without quotes. |
| `` a Real or BS Video needs `tested` `` | `tested` is missing or empty. | Say in one line what was tested. |
| `is only for Real or BS Videos` | `verdict`, `episode` or `tested` is on a Video of another Format. | Remove the line, or set `format = "Real or BS"`. |
| `matches no Playbook Entry` | `playbook` names a file that is not in `content/playbooks/`. | Write it like `playbooks/<name>.md`, and add that Playbook Entry. |
| `the Video has no Clip` | `clip` is missing. | Make the Clip (see [Add the Clip](#add-the-clip)) and set `clip`. |
| `` `clip` must be the path of an MP4 `` | `clip` is not written like `clips/<name>.mp4`. | Use the line the Clip command printed. |
| `the Clip file static/... does not exist` | No file at that path under `static/`. | Run the Clip command, or correct the path. |
| `over the budget of 400000 bytes` | The Clip weighs more than 400 KB. | Make it again with the Clip command; it stays well under. |
| `the Video has no still` | `still` is missing. | Set `still` to the WebP the Clip command wrote with the Clip. |
| `` `still` must be the path of a WebP `` | `still` is not written like `clips/<name>.webp`. | Use the line the Clip command printed. |
| `the still file static/... does not exist` | No file at that path under `static/`. | Run the Clip command, or correct the path. |
| `the Playbook has no page and no PDF` | The Playbook has nothing below its front matter and no `pdf`. | Write the Playbook below the front matter (template 4), or set `pdf` to the https address of the PDF (template 3). |

Three things look different:

- **A missing or heavy Clip.** Zola cannot see an MP4, so these two refusals come from the test
  `the_real_videos_all_have_a_clip_under_budget` in `just check`, not from the build.
- **No date.** Zola leaves an Entry with no date out of the site with only a warning, so this
  refusal comes from the test `the_real_entries_all_have_a_date` in `just check`, not from the
  build. The message is the same.
- **Front matter that is not valid TOML** (a missing quote, a date like `8 November`). Zola refuses
  it with its own message, which names the file and the line.
