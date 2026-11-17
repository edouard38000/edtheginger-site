+++
title = "How I won a Claude hackathon with no team and without writing code"
date = 2026-11-13
description = "Pick the idea, build it with parallel agents and pitch it, step by step."

[extra]
cover = "covers/hackathon-win-playbook.jpg"
+++
I can't write code and I entered alone. In under 7 hours, about 30 Claude Code agents running in parallel built BoldFresh, an app that turns a cooking video into a supermarket basket, with an onboarding, a live demo and real prices. It won.

{{<shots srcs={["playbooks/hackathon-win/hackathon_photos.jpg"]} alts={["Two photos from the hackathon: four people standing on a stage in a stone cellar, and a close-up of one of them smiling with their hands together."]} />}}

{{<subhead text="What you need" level={2} />}}

{%<row key="A paid Claude plan">%}Pro, Max and Ultra all include Claude Code. Pro is $20 a month if you pay monthly. My link below gives you a free week of Claude Code, while my guest passes last.{%</row>%}

{%<row key="The Claude desktop app">%}Claude Code runs inside it, under Code, on Mac or Windows.{%</row>%}

{%<row key="A free GitHub account">%}Your agents save their work there, one branch per task.{%</row>%}

{%<row key="One problem you have felt">%}[Part 1](#part-1) shows how to pick it.{%</row>%}

{{<buttons texts={["Free week of Claude Code", "Download Claude"]} urls={["https://claude.ai/referral/xylnBopBlw", "https://claude.com/download"]} />}}

{{<subhead text="The three parts" level={2} />}}

1. [**The idea.**](#part-1) Pick a real problem you have felt yourself, research it, and say it in one sentence.
2. [**The build.**](#part-2) Reuse what already exists, plan in detail, then give one small task to each agent, many at once.
3. [**The pitch.**](#part-3) A live demo that makes the room react, a clean design, and one sentence everyone understands.

{%<note>%}Prices checked 13 November 2026 on claude.com/pricing, in US dollars.{%</note>%}

{%<part n={1} id="part-1" title="The idea">%}
Picking the idea is harder than it looks. Judges reward a problem they recognise and a demo that solves it. The strongest ideas come from a problem you have lived, because then you already know what the fix has to feel like.
{%</part>%}

{{<label text="MY PROBLEM: SAVED, NEVER COOKED" />}}

{{<shots srcs={["playbooks/hackathon-win/idea_saved.jpg", "playbooks/hackathon-win/idea_paywall.jpg"]} alts={["A grid of saved cooking videos from Instagram and TikTok.", "A recipe website hiding the recipe behind a pop-up: Unlock all of our Recipes, Free for 14 Days."]} />}}

{%<step n={1} title="List problems you have had yourself">%}Write down three things that annoy you every week. Mine: I saved cooking videos on Instagram and TikTok and never cooked them. I was too lazy, the recipe was paywalled, or the video got lost among thousands of saves.{%</step>%}

{%<step n={2} title="Research each one">%}Before you choose, find out who has the problem, how they deal with it today and what the apps that already exist get wrong. Claude's Research mode or ChatGPT's deep research does this in a few minutes.{%</step>%}

{%<step n={3} title="Pick the one you can demo">%}The best pick is a real problem, felt by many people in the room, that a 3-minute live demo can show solved. Drop anything you can't build in the hours you have.{%</step>%}

{%<step n={4} title="Say it in one sentence">%}Write the promise now and keep it for the pitch. Mine: "My app turns any cooking video you scroll past into a full basket at your favourite supermarket in less than two clicks."{%</step>%}

{{<subhead text="Four questions to research" icon="lightbulb" />}}

{%<row key="Who has it?">%}A group you can name and picture, and roughly how often they hit the problem. "People who save recipe videos" beats "everyone who eats".{%</row>%}

{%<row key="How do they cope today?">%}Screenshots, notes apps, bookmarks, giving up. The workaround shows what your app has to replace.{%</row>%}

{%<row key="What do current apps miss?">%}Read the one-star reviews of the closest apps. What people complain about there is what your app has to do better.{%</row>%}

{%<row key="Can you show it in 3 minutes?">%}If the result only appears after weeks of use, it's a bad hackathon idea, however good it is.{%</row>%}

{%<part n={2} id="part-2" title="The build: plan">%}
Spend more time planning than coding. Agents build fast when they know exactly what to build, and they make a mess when they guess. First reuse what already exists, then turn the idea into small, precise tickets.
{%</part>%}

{{<step_image src="playbooks/hackathon-win/plan_trending.jpg" label="WHERE I FOUND MY PARTS" alt="The GitHub Trending page, listing the repositories people are starring today." />}}

{{<step_image src="playbooks/hackathon-win/plan_skills.jpg" alt="Matt Pocock's skills page, listing the engineering skills he uses daily: ask-matt, grill-with-docs, triage, improve-codebase-architecture, setup-matt-pocock-skills, to-spec, to-tickets and implement." />}}

{%<step n={1} title="Reuse open-source parts">%}Most of your app already exists somewhere: a video downloader, a recipe parser, a login. Ask Claude Code to find open-source projects for each part before it writes anything, and to check their licence and when they were last updated.{%</step>%}

{%<step n={2} title="Know where to look">%}**GitHub Trending** shows the projects people are using right now. **Awesome Claude Code** lists plugins, skills and tools made for Claude Code.{%</step>%}

{%<step n={3} title="Plan with coding skills">%}I plan with Matt Pocock's free skills: small instruction files that teach Claude Code a way of working. His page shows the one-line install.{%</step>%}

{%<step n={4} title="Get grilled, then write the plan">%}**/grill-with-docs** asks you question after question until every choice is made. **/to-spec** writes the answers up as a spec, and **/to-tickets** splits the spec into small tickets that say which ones block which.{%</step>%}

{{<buttons texts={["GitHub Trending", "Awesome Claude Code", "Matt Pocock's skills"]} urls={["https://github.com/trending", "https://github.com/hesreallyhim/awesome-claude-code", "https://github.com/mattpocock/skills"]} />}}

{{<subhead text="What a good ticket looks like" icon="check" />}}

{%<row key="Small">%}One agent can finish it in well under an hour. If not, split it again.{%</row>%}

{%<row key="Testable">%}It says how to check that it works, so the agent can test its own work.{%</row>%}

{%<row key="Clear about blockers">%}It names the tickets that must be done first, so agents start in the right order.{%</row>%}

{%<row key="Owns its files">%}Two tickets that change the same files can't run at the same time. Split them differently, or run them one after the other.{%</row>%}

{%<part n={2} id="part-2-agents" title="The build: agents">%}
The app got built in under 7 hours because every ticket went to its own agent, and many agents worked at the same time, each in a separate copy of the project so they never overwrote each other.
{%</part>%}

{{<step_image src="playbooks/hackathon-win/agents_card.jpg" label="MY 30 AGENTS" alt="A slide: How we built it, today. Built by ~30 parallel Claude Code agents in one day. 100+ PRs, ~330 tests, 1 binary." />}}

{{<step_image src="playbooks/hackathon-win/agents_worktrees.jpg" alt="A terminal listing .claude/worktrees: one folder per agent, from agent-a037c9a6bb0fdd57c to agent-af842e9e83db65eac." />}}

{%<step n={1} title="One small task per agent">%}A ticket an agent can finish and test on its own works far better than a vague, big one. Claude Code can start subagents: separate agents it hands one task each.{%</step>%}

{%<step n={2} title="Run them in parallel">%}Ask Claude Code to start one subagent per ticket that is ready, each in its own git worktree: a separate working copy of the project on its own branch. The list above is my agents' worktrees.{%</step>%}

{%<step n={3} title="Review before you merge">%}Each agent opens a pull request, and every one gets a review against the spec before it is merged. Matt Pocock's /code-review skill does this. My build ended with more than 100 pull requests and about 330 tests.{%</step>%}

{%<step n={4} title="Keep a working demo">%}After every merge, run the whole app end to end. Fix what broke before starting new tickets, so you always have something you could demo.{%</step>%}

{{<subhead text="What to watch out for" icon="exclamation" />}}

{%<row key="Usage limits">%}Thirty agents use your plan's limits much faster than one chat. Max gives you more room than Pro.{%</row>%}

{%<row key="Approvals">%}Claude Code asks before it runs commands or edits files. Read what it asks, and approve only what you understand.{%</row>%}

{%<row key="Agents that guess">%}When an agent builds something odd, the ticket was usually too vague. Rewrite the ticket and run it again.{%</row>%}

{%<row key="Too many at once">%}Start with a few agents, check that their work merges cleanly, then add more.{%</row>%}

{%<part n={3} id="part-3" title="The pitch">%}
People spend the least time on the pitch, and they shouldn't. The judges only see what you show in a few minutes, so give it real hours.
{%</part>%}

{{<label text="MY PITCH ON STAGE" />}}

{{<shots srcs={["playbooks/hackathon-win/pitch_stage.jpg"]} alts={["The pitch on stage in a dark stone cellar, with the BoldFresh slide on a screen."]} />}}

{{<step_image src="playbooks/hackathon-win/pitch_slide.jpg" alt="Three slides, Share, Plan and Shop, each with a phone screen of the app. Live demo on stage, on the real app." />}}

{%<step n={1} title="Make the live demo impressive">%}Show the real app, live, not a recording. Rehearse the exact clicks until the demo works every time.{%</step>%}

{%<step n={2} title="Interact with the audience">%}Ask the room for something the demo can use, like a link or a choice. People pay attention when the demo is about them.{%</step>%}

{%<step n={3} title="Show every feature end to end">%}Go from the first screen to the result in one go. Build up to the moment you want the room to say wow.{%</step>%}

{%<step n={4} title="Make it simple">%}Everyone in the room should understand what you built, including people outside tech. Say the one-sentence promise, and let three slides carry the rest.{%</step>%}

{{<subhead text="A 3-minute pitch, in five beats" icon="clock" />}}

{%<row key="The problem">%}A moment everyone in the room has lived, told in 20 seconds. Mine: the recipe videos we all save and never cook.{%</row>%}

{%<row key="The promise">%}Your one sentence, said slowly.{%</row>%}

{%<row key="The live demo">%}Click by click, from the first screen to the result, with one moment where the room takes part.{%</row>%}

{%<row key="How it was built">%}One line. Mine: about 30 Claude Code agents in under 7 hours.{%</row>%}

{%<row key="The close">%}Repeat the promise. Keep a backup ready for each demo step in case the internet or the app fails.{%</row>%}

{%<part n={4} id="part-4" title="The design and the day">%}
Small tip: if you have time to work on the design and the user experience, it gives you bonus points. It matters more than most people think.
{%</part>%}

{{<shots srcs={["playbooks/hackathon-win/design_screens.jpg"]} labels={["THE SCREENS I DESIGNED"]} alts={["A collage of the BoldFresh screens: Save, the recipe catalogue, a recipe preview, a recipe with its ingredients and supermarket products, and Your first basket in 20 seconds."]} />}}

{%<step n={1} title="Pick one look and keep it">%}One main colour, one font, rounded cards, real pictures. Ask Claude Code to apply the same style to every screen, then compare the screens side by side.{%</step>%}

{%<step n={2} title="Design the first screen first">%}The onboarding and the home screen are what the room sees first. Make them clean before you polish anything else.{%</step>%}

{%<step n={3} title="Use real content">%}Real recipes with real prices and photos make the demo believable, and placeholder text makes it look unfinished.{%</step>%}

{%<step n={4} title="Check it on the device you demo on">%}If you present from a phone, test every screen on that phone, at the size the room will see it.{%</step>%}

{{<subhead text="A simple plan for the day" icon="clock" />}}

{%<row key="Before you arrive">%}Pick the idea, research it and write the one-sentence promise. Install Claude Code and the planning skills.{%</row>%}

{%<row key="First hours">%}Look for parts to reuse, then grill, spec and tickets. Don't start building until the tickets are small.{%</row>%}

{%<row key="Middle hours">%}Agents in parallel, one ticket each. Review, merge, and keep the app working end to end.{%</row>%}

{%<row key="Last hours">%}Stop adding features. Polish the design, write and rehearse the pitch, and prepare a backup for the demo.{%</row>%}
