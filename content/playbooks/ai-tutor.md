+++
title = "How to get Jane Street's AI tutor on your computer"
date = 2026-11-19
description = "Install Jane Street's free tutor in Claude, then learn any subject by answering its questions."

[extra]
cover = "covers/ai-tutor-playbook.jpg"
+++
Jane Street is a big Wall Street trading firm. It put an AI tutor online for free: a small add-on for Claude, called socratic-tutor. You tell it what you want to learn. It splits the subject into parts, asks you a few questions to find your level, then teaches you part by part, by asking you questions instead of giving answers. In the video I used it for 15 minutes to learn how AI chatbots work. Here is every step I followed, in order.

{{<subhead text="What you need" icon="check" level={2} />}}

{%<row key="A computer">%}A Mac or a Windows PC. I used a Mac. I have not tried it on Windows.{%</row>%}

{%<row key="The Claude app">%}Free to download from claude.com/download.{%</row>%}

{%<row key="A paid Claude plan">%}The tutor runs in Claude Code, the Code tab of the Claude app, and the free plan does not include it. Pro costs $20 a month in the US. My link below gives you a free week, while my guest passes last.{%</row>%}

{%<row key="5 minutes">%}About 30 seconds to install the tutor, then as long as you want to learn.{%</row>%}

{{<buttons texts={["Free week of Claude Code", "Get the Claude app", "The tutor on GitHub"]} urls={["https://claude.ai/referral/xylnBopBlw", "https://claude.com/download", "https://github.com/janestreet/agent_skills"]} />}}

{{<subhead text="The four parts" level={2} />}}

1. [**Open Claude Code.**](#part-1) The Code tab of the Claude app.
2. [**Install the tutor.**](#part-2) One sentence and a link.
3. [**Start a lesson.**](#part-3) Name the subject and answer the first questions.
4. [**Read your map and learn.**](#part-4) Pick a part and keep answering.

{%<part n={1} id="part-1" title="Open Claude Code">%}
Claude Code is the part of the Claude app that can do things on your computer, like downloading a file or saving it in the right folder. The tutor needs it. Claude was already on my computer, so I did not film the download: the first two steps come from Anthropic's own help pages.
{%</part>%}

{%<step n={1} title="Download the Claude app">%}Go to claude.com/download, download the app for your computer, open it and sign in with your Claude account.{%</step>%}

{%<step n={2} title="Make sure your plan is paid">%}If you are on the free plan, the app asks you to upgrade when you open Code. Use my link above for a free week.{%</step>%}

{%<step n={3} title="Click Code">%}At the top left of the app there are two small buttons. Click the one that looks like this: &lt;/&gt;. That opens Claude Code.{%</step>%}

{%<step n={4} title="Start a new session">%}Click **+ New**. A session is a fresh conversation with Claude Code.{%</step>%}

{{<step_image src="playbooks/ai-tutor/code_tab.jpg" label="THE CODE BUTTON" alt="The top left of the Claude app: the orange cursor clicks the Code button, next to the Chat button, above New, Search, Projects and Artifacts." />}}

{%<part n={2} id="part-2" title="Install the tutor">%}
You don't install anything by hand. You paste one sentence and the link to the tutor, and Claude downloads it and adds it to your Claude.
{%</part>%}

{%<step n={1} title="Paste the prompt below">%}Paste it in the message box at the bottom of the session and press Enter. Keep the link on its own line.{%</step>%}

{%<step n={2} title="Say yes when Claude asks">%}Claude asks for permission before it downloads or saves anything. Read what it asks and allow it.{%</step>%}

{%<step n={3} title="Wait for the confirmation">%}Mine took about 25 seconds. Claude answered: "The repo is cloned and its skill is installed and working in this session."{%</step>%}

{%<prompt label="PROMPT" dest="paste in Claude Code" logos="claude">%}
Can you please clone the repo below on my computer and install the skill in this session?

https://github.com/janestreet/agent_skills
{%</prompt>%}

{{<step_image src="playbooks/ai-tutor/installed.jpg" label="WHAT I GOT" alt="Claude's answer, framed in red: The repo is cloned and its skill is installed and working in this session." />}}

{%<note>%}Clone means download a copy. A repo is a folder of files on GitHub, here Jane Street's tutor. A skill is a set of instructions Claude follows, and socratic-tutor is the name of this one. If a window on your Mac asks you to install developer tools, click Install, wait, and paste the prompt again. That did not happen to me.{%</note>%}

{%<part n={3} id="part-3" title="Start a lesson">%}
To use the tutor, you call it by its name. Type a slash, its name, then what you want to learn.
{%</part>%}

{%<step n={1} title="Type the prompt below">%}Swap "how AI chatbots actually work" for anything you want to learn. Keep "/socratic-tutor" at the start: it is what starts the tutor.{%</step>%}

{%<step n={2} title="Read how it splits the subject">%}It lists the parts it will teach. For how AI chatbots work, it made 9 parts, from next-token prediction to hallucinations.{%</step>%}

{%<step n={3} title="Answer its first questions">%}It asks one question at a time, about 5 in all, to find your level. It teaches nothing yet. Answer in your own words. "No idea" is a fine answer: I said it once and it simply moved on.{%</step>%}

{%<prompt label="PROMPT" dest="paste in Claude Code" logos="claude">%}
/socratic-tutor I want to learn how AI chatbots actually work.
{%</prompt>%}


{%<part n={4} id="part-4" title="Read your map and learn">%}
After the first questions, the tutor shows a map: every part of the subject with a score from 0 to 5, and an overall percentage. 0 means you have never heard of it, 5 means you could teach it. Mine started at 40%.
{%</part>%}

{%<step n={1} title="Pick where to start">%}The map marks the part it suggests. Reply with the one you want. I wrote "start with pre training".{%</step>%}

{%<step n={2} title="Keep answering">%}It teaches with one question per message and builds on your answers. It explains only once you are stuck, after a couple of tries.{%</step>%}

{%<step n={3} title="Watch your scores">%}A score only goes up when you show you understand. After my 15 minutes, pre-training went from 1 to 2 and my overall from 40% to 42%.{%</step>%}

{{<step_image src="playbooks/ai-tutor/skill_map.jpg" label="MY MAP" alt="The tutor's map for How AI chatbots work: 9 parts with bars and scores out of 5, overall 40%, Pretraining at 1 out of 5 marked as the proposed focus." />}}

{{<subhead text="Things you can type during a lesson" icon="lightbulb" />}}

{%<row key="show progress">%}Shows your map again.{%</row>%}

{%<row key="easier / harder">%}Changes the level of the questions.{%</row>%}

{%<row key="just tell me">%}Gives you the answer to the current question, then asks a new question on the same idea. Your score does not go up for it.{%</row>%}

{%<row key="save state">%}Gives you a block of text that holds your progress. To carry on another day, start a new session and paste it after the words "resume from:". I have not tried resuming yet.{%</row>%}
