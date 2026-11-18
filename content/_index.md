+++
title = "Edouard the Ginger"
template = "index.html"

[extra]
# The Hero's tagline, under EDOUARD | THE GINGER: the one place its wording is held.
tagline = "Learn AI without the hype, only the facts"
# My Story: the story itself is the body of this file, below the front matter (Markdown). Until
# there is one, the section shows this line under the Poster.
story_coming_soon = "My story is coming soon, in my own words."

# The Poster's captions: the one place their wording is held.
# Shown together as a small block in the top left corner.
captions = [
  "I test viral AI claims",
  "real or bullshit, with the playbook",
  "short AI and tech videos",
]
# The portrait on the Poster, a file under static/. Now the AI relight of Edouard's photo
# (source: brand-assets/portrait/portrait-relit-ai.png). To go back to the scripted light, run
# scripts/light-portrait and set this to "home/portrait-lit.webp".
portrait = "home/portrait-relit-ai.webp"
# Alt text of the portrait on the Poster.
portrait_alt = "Edouard the Ginger in the dark, warm light falling across his forehead and eyes"

# The Hero: a short muted video and its poster frame, as two pairs of files under static/. The tall
# pair (9:16) plays on portrait screens (phones), the wide pair (16:9) on landscape screens
# (computers); each visitor downloads only one video. The stand-ins were made from the portrait by
# brand-assets/hero/make-stand-in-hero.sh.
# The real Hero arrives as one 16:9 recording plus a 9:16 crop of its centre: replace both pairs
# together, so phones and computers show the same footage. Overwrite the four files (or point
# these four lines at new ones): each video H.264 MP4 with no audio track and fast-start, under
# 1.5 MB; each poster its video's first frame as WebP.
[extra.hero]
video = "home/hero.mp4"
poster = "home/hero-poster.webp"
wide_video = "home/hero-wide.mp4"
wide_poster = "home/hero-wide-poster.webp"

# The light across the face, in pixels of the portrait above (1350 x 1693): a broad, soft diagonal
# band over the forehead, eyes and nose, falling towards the lower right. The two white lines
# are its upper and lower edges, placed where the light visibly starts and ends (measured on the
# image). scripts/light-portrait lights the cut-out with the same numbers when the scripted
# portrait is in use (they must then be measured on that one).
[extra.beam]
x = 675       # a point on the middle of the band, between the eyes
y = 664
slope = 0.55  # how far the lines drop per pixel to the right
width = 231   # vertical distance between the two lines
soft = 5      # used by scripts/light-portrait only

# The tools shown as tiles around the portrait: the one list, most important first.
# A phone shows the first 8. `logo` is a file in static/home/logos/ (sources: brand-assets/logos/README.md).
# To add a tool, add its logo file and one block here; the layout places it by its rank.
[[extra.tools]]
name = "Claude"
logo = "claude.svg"

[[extra.tools]]
name = "ChatGPT"
logo = "chatgpt.svg"

[[extra.tools]]
name = "GitHub"
logo = "github.svg"

[[extra.tools]]
name = "n8n"
logo = "n8n.svg"

[[extra.tools]]
name = "CapCut"
logo = "capcut.svg"

[[extra.tools]]
name = "ElevenLabs"
logo = "elevenlabs.svg"

[[extra.tools]]
name = "Suno"
logo = "suno.svg"

[[extra.tools]]
name = "Gemini"
logo = "gemini.svg"

[[extra.tools]]
name = "Perplexity"
logo = "perplexity.svg"

[[extra.tools]]
name = "Canva"
logo = "canva.svg"

[[extra.tools]]
name = "Notion"
logo = "notion.svg"
+++
