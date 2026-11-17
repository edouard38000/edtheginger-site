+++
title = "Edouard the Ginger"
template = "index.html"

[extra]
# The Poster's captions: the one place their wording is held.
# Shown together as a small block in the top left corner.
captions = [
  "I test viral AI claims",
  "real or bullshit, with the playbook",
  "short AI and tech videos",
]
# Alt text of the portrait on the Poster.
portrait_alt = "Edouard the Ginger in shadow, a band of light across his eyes, arms crossed"

# The beam of light across the face: the strip between the two white diagonal lines. The same
# numbers light the portrait (scripts/light-portrait, run it again after a change) and draw the
# lines in the template. In pixels of brand-assets/portrait/portrait-cutout.png (1350 x 1800).
[extra.beam]
x = 684       # a point on the middle of the beam, between the eyes
y = 582
slope = 0.36  # how far the lines drop per pixel to the right
width = 100   # vertical distance between the two lines
soft = 5      # pixels the light takes to fade out at each line

# The two straight lines framing the face, at eye and nose height (same pixels).
[extra.frame_lines]
eyes = 582
nose = 645

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
