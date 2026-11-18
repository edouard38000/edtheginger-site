# Glossary

The words this site and its tickets use. One meaning each.

- **Video**: one short video Edouard posted, shown on the site as a cover or a Clip with links out to TikTok and Instagram. Never an embedded TikTok or Instagram player (see `docs/adr/0001-clips-not-embedded-players.md`).
- **Clip**: a silent five-second loop cut from the start of a Video (its hook), hosted on the site. A Clip is a preview: tapping it opens the Video on TikTok or Instagram.
- **Format**: the series a Video belongs to (Real or BS, GitHub tools, Storytelling, Talking head).
- **Verdict**: the outcome of a Real or BS Video: `real` or `bs`. Only Real or BS Videos have one.
- **Tally**: the two counts `REAL: n  BS: n`, always computed from the Verdicts, never typed by hand.
- **REAL list**: the Real or BS Videos whose Verdict is `real`, numbered from 1 in posting order and shown newest first, each with its Playbook.
- **Playbook**: the step-by-step guide for one Video. A Playbook is a page on the site (`/playbooks/<name>/`), written in Markdown in its Entry. An older Playbook can be only a PDF (a **legacy PDF**): it stays at its existing address on the playbook site, this site links to it and never moves it. A Playbook page may keep its legacy PDF in its Entry, but the site no longer links to it.
- **Entry**: one content file describing one Video or one Playbook (for a Playbook page, the file also holds the page itself). Adding an Entry is the only step needed to put something on the site.
- **Dock**: the fixed row of links to Instagram, TikTok and the contact email.
- **Hero**: the top of the home page: a short muted video of Edouard's face cutting fast behind his name and tagline, the first thing a visitor sees.
- **Wheel**: the home page section below the Hero: the Clips of the eight newest Videos on cards fanned round a turning wheel.
- **Ed's Daily**: Edouard's daily newsletter, sent through a pipeline he owns. The home page has its signup.
- **My Story**: the last section of the home page, Edouard's story in his own words, opened by the Poster.
- **Poster**: Edouard's portrait with the title, small captions and his tool tiles. It opens the My Story section.
- **Desktop layout**: the Videos section on wide screens, with covers scattered like icons around the blurred portrait. On phones it is a plain grid.
