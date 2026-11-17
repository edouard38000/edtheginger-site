+++
title = 'Fixture: the "Field" Guide'
date = 2026-01-30
description = "Fixture: a Playbook page that uses every component."

[extra]
cover = "covers/fixture-cover.png"
+++
Fixture intro: what the reader gets.

{{<subhead text="What you need" icon="check" level={2} />}}

{%<row key="A phone">%}Any phone, see [part 2](#part-2).{%</row>%}

{{<buttons texts={["Free week of Fixture", "Fixture on GitHub"]} urls={["https://example.com/free-week", "https://example.com/github"]} />}}

{{<subhead text="The two parts" level={2} />}}

1. [**Start.**](#part-1) The first part.
2. [**Finish.**](#part-2) The second part.

{%<part n={1} id="part-1" title="Start here">%}
The intro of part 1.
{%</part>%}

{{<label text="THIS ERASES EVERYTHING" />}}

{{<subhead text="On your computer" icon="lightbulb" />}}

{%<step n={1} title='Say "hello"'>%}Type **hello** and press Enter.{%</step>%}

{%<step n={2} title="Paste the prompt">%}It is below.{%</step>%}

{%<prompt label="PROMPT 1">%}
Find the best coffee in [YOUR CITY] & tell me "why", under [A PRICE] <today>.
{%</prompt>%}

{{<skill name="fixture-skill" url="https://example.com/fixture-skill.zip" />}}

{%<prompt label="PROMPT 2" dest="paste in Claude Code" logos="claude" example_label="One of mine:" example="Find a cafe in Lyon.">%}
Open the app so I can pair my phone.
{%</prompt>%}

{{<step_image src="covers/fixture-cover.png" label="WHAT IT LOOKED LIKE" alt="Fixture: the wide screenshot." caption="Fixture caption." />}}

{%<note>%}Fixture note.{%</note>%}

{%<part n={2} id="part-2" title="Finish">%}
The intro of part 2.
{%</part>%}

{{<shots srcs={["covers/fixture-cover.png", "covers/fixture-cover.png"]} labels={["01 ONE", "02 TWO"]} alts={["Fixture: the first phone screen.", "Fixture: the second phone screen."]} />}}

{{<skill name="no-link-skill" />}}
