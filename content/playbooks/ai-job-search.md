+++
title = "Find a job with AI: from CV to interview"
date = 2026-10-23
description = "Use AI for each step of a job search, from the CV to the interview."

[extra]
cover = "covers/ai-job-search-playbook.jpg"
+++
AI Job Search is a free, open-source framework that runs inside Claude Code on your computer. It builds a profile from your CV, searches job boards, scores each match and drafts a tailored CV and cover letter for the jobs you choose. You review every document and send each application yourself.

{{<subhead text="What you need" icon="check" level={2} />}}

{%<row key="Claude Pro or Max">%}Both plans include Claude Code.{%</row>%}

{%<row key="A Mac or Windows PC">%}Linux also works, with the terminal version of Claude Code.{%</row>%}

{%<row key="Your CV as a PDF">%}Add your LinkedIn profile as a PDF too if you have one.{%</row>%}

{%<row key="Disk space for LaTeX">%}MacTeX is a download of several gigabytes. TinyTeX is much smaller and needs a few extra packages, listed in SETUP.md.{%</row>%}

{{<subhead text="The six parts" level={2} />}}

1. [**Get Claude Code.**](#part-1) Claude Pro and the desktop app.
2. [**Install.**](#part-2) Download the framework.
3. [**Profile.**](#part-3) Build your profile with /setup.
4. [**Search.**](#part-4) Find and rank jobs with /scrape.
5. [**Apply.**](#part-5) Draft each application with /apply.
6. [**Interview.**](#part-6) Prepare interviews and follow up.

{%<part n={1} id="part-1" title="Get Claude Code">%}
{%</part>%}

{{<step_image src="playbooks/ai-job-search/claude-code.jpg" caption="Claude Pro · desktop app" alt="A pixel-art Claude Pro card above a computer screen with an orange tick." />}}

{%<step n={1} title="Subscribe to Claude Pro">%}Claude Code has no free plan. Pro costs $20 a month, or $200 a year paid up front. Max starts at $100 a month if you need more usage than Pro allows.{%</step>%}

{%<step n={2} title="Install the desktop app">%}Download Claude for macOS or Windows and sign in with the account you subscribed with. Claude Code runs inside the app, under Code.{%</step>%}

{%<step n={3} title="Know what Claude Code does">%}Claude Code is an AI agent that reads and writes files and runs commands on your computer. By default it asks before it edits a file or runs a command, so read each request before you approve it.{%</step>%}

{{<buttons texts={["Claude pricing", "Download Claude"]} urls={["https://claude.com/pricing", "https://claude.com/download"]} />}}

{%<note>%}Prices checked 23 October 2026 on claude.com/pricing, in US dollars. AI Job Search is an independent project by Mads Lorentzen, released under the MIT licence and not made by Anthropic: [**AI Job Search on GitHub**](https://github.com/ai-job-search-dev/ai-job-search).{%</note>%}

{%<part n={2} id="part-2" title="Download the framework" label="Install">%}
Claude Code downloads the project and installs what it needs: Python 3.10 or newer, Bun to run the job-board searches, and LaTeX to turn your CV and cover letter into PDFs.
{%</part>%}

{{<step_image src="playbooks/ai-job-search/install.jpg" label="PASTE THE LINK" caption="GitHub · your computer" alt="A pixel-art link card reading github.com/MadsLorentzen/ai-job-search, a folder on a computer screen with a tick, and a terminal running claude." />}}

{%<step n={1} title="Start a session">%}In the desktop app, open Code, start a new session and pick the folder where the project should go, such as Documents.{%</step>%}

{%<step n={2} title="Paste the prompt">%}Copy the prompt below, replace <span class="pb-hot">[MAC OR WINDOWS]</span> and send it. Read each install request before you approve it. LaTeX takes the longest.{%</step>%}

{%<step n={3} title="Reopen in the project folder">%}When Claude reports that everything works, start a new session in the new ai-job-search folder. **/setup**, **/scrape** and **/apply** only exist in sessions opened there.{%</step>%}

{%<prompt label="PROMPT" dest="paste in Claude Code" logos="claude">%}
Download the AI Job Search framework from https://github.com/ai-job-search-dev/ai-job-search into this folder. Clone it; do not fork it, and never push anything to GitHub.

Then install what its README lists for [MAC OR WINDOWS] that I don't already have: Python 3.10 or newer, Bun, a LaTeX distribution with lualatex and xelatex, and the optional pypdf package.

Run tools/install_portal_clis.py with Python to install the job-search tools.

Before each install command, tell me in one sentence what it does. When everything works, tell me to open the ai-job-search folder in a new session and run /setup.
{%</prompt>%}

{%<note>%}The link in the video, github.com/MadsLorentzen/ai-job-search, now redirects to this address.{%</note>%}

**Keep your copy private.** /setup writes your name, contact details, work history and salary expectations into the project's files. A GitHub fork of this project is always public, which is why the prompt clones it to your computer instead. For an online backup, use a private repository as described in [**SETUP.md, section 8**](https://github.com/ai-job-search-dev/ai-job-search/blob/master/SETUP.md#8-pulling-upstream-updates-into-your-fork).

{%<part n={3} id="part-3" title="Build your profile with /setup" label="Profile">%}
/setup turns your documents into the profile files that every later step reads. The more documents you give it, the less you have to type.
{%</part>%}

{{<step_image src="playbooks/ai-job-search/profile.jpg" label="FILL IN YOUR INFO" caption="/setup" alt="A pixel-art CV, a terminal bubble reading /setup with a tick, and a person." />}}

{%<step n={1} title="Add your documents">%}In the ai-job-search folder, put your CV in documents/cv and your LinkedIn PDF in documents/linkedin. Diplomas, reference letters and past applications go in documents/diplomas, documents/references and documents/applications. For the LinkedIn PDF, open your profile and choose More, then Save to PDF.{%</step>%}

{%<step n={2} title="Run /setup">%}Type **/setup** and press Enter. Choose A to read the documents folder, B to paste a single CV, or C to answer questions section by section.{%</step>%}

{%<step n={3} title="Check the result">%}Paste the prompt below and fix anything wrong or missing. Later drafts only use facts that are in your profile.{%</step>%}

{%<prompt label="PROMPT" dest="paste in Claude Code" logos="claude">%}
Compare my profile files with everything in documents/. List each claim in my profile that no document supports, and each important fact in my documents that the profile leaves out, such as a skill, result or responsibility.

Don't change any file yet; wait for my corrections.
{%</prompt>%}

Run **/setup** again whenever you add documents; reading the documents folder is safe to repeat. **/expand** adds skills it finds through public links in your profile, such as a GitHub account or a portfolio.

{%<part n={4} id="part-4" title="Find and rank jobs with /scrape" label="Search">%}
/scrape searches job boards for roles that fit your profile, removes duplicates and lists the matches with a fit score.
{%</part>%}

{{<step_image src="playbooks/ai-job-search/search.jpg" label="GET YOUR MATCHES" caption="/scrape · /rank" alt="A pixel-art phone listing Your matches: Product Analyst 94%, Data Scientist 89%, Marketing Manager 81%, UX Researcher 76% and Strategy Associate 72%, under five job cards with the same scores." />}}

{%<step n={1} title="Run /scrape">%}Type **/scrape**. It searches with your profile and lists the matches sorted by fit.{%</step>%}

{%<step n={2} title="Know which boards it covers">%}It ships with LinkedIn's public job listings and freehire.me, which work across countries, plus four Danish boards: Jobindex, Jobnet, Akademikernes Jobbank and Jobdanmark.{%</step>%}

{%<step n={3} title="Add your country's board">%}Type **/add-portal** followed by the board's address. Claude studies the site, builds a search tool for it and tests a live search before adding it.{%</step>%}

{%<step n={4} title="Rank a long list">%}**/rank** scores every new match on five criteria and returns a shortlist with strengths, gaps and deadlines. Deal-breakers in your profile rule a job out.{%</step>%}

{%<prompt label="COMMANDS" dest="type in Claude Code" logos="claude">%}
/scrape

/add-portal [JOB BOARD ADDRESS]

/rank
{%</prompt>%}

Some sites block automated access. If a search or a job link fails, open the posting yourself and paste its text after **/apply** ([next part](#part-5)).

{%<part n={5} id="part-5" title="Draft each application with /apply" label="Apply">%}
/apply writes a CV and a cover letter for one job, has a second agent review them and compiles both into PDFs.
{%</part>%}

{{<step_image src="playbooks/ai-job-search/apply.jpg" label="TAILORED TO THE JOB" caption="/apply" alt="Two pixel-art documents, a CV and a cover letter, joined by an orange tick." />}}

{%<step n={1} title="Choose a job">%}After /scrape or /rank, pick a match from the list. You can also run **/apply** with a job link, or with the full job description pasted after it when the site blocks the link.{%</step>%}

{%<step n={2} title="Let it work">%}It scores the fit, drafts both documents, sends them to a reviewer agent that researches the company, revises them and compiles the PDFs: a two-page CV and a one-page letter. It also checks that an applicant tracking system (ATS), the software many employers use to screen CVs, can read your CV's text.{%</step>%}

{%<step n={3} title="Review, then send it yourself">%}Read both PDFs before you send anything. The framework only uses facts from your profile, but check every claim, the company name and the job title. Then apply on the employer's site.{%</step>%}

{%<prompt label="COMMANDS" dest="type in Claude Code" logos="claude">%}
/apply [JOB LINK]

/apply [FULL JOB DESCRIPTION]
{%</prompt>%}

Job postings can hide instructions aimed at AI tools. The framework treats a posting as data and ignores those instructions, but skim what it fetched before you send, especially from an unfamiliar job board.

{%<part n={6} id="part-6" title="Prepare interviews and follow up" label="Interview">%}
Spend the time you save on interviews. These commands track each application and prepare you for the next conversation.
{%</part>%}

{{<step_image src="playbooks/ai-job-search/interview.jpg" label="PREPARE THE INTERVIEW" caption="/interview · /outcome" alt="Two pixel-art people talking in speech bubbles, with a crossed-out hourglass between them." />}}

{%<step n={1} title="Record each outcome">%}**/outcome** logs interview stages, offers, rejections and silence, archives the documents you sent and updates job_search_tracker.csv.{%</step>%}

{%<step n={2} title="Follow up on quiet applications">%}**/outcome followup** lists applications with no news for 10 days and drafts a short follow-up in your writing style. It never sends anything and drafts at most two per application.{%</step>%}

{%<step n={3} title="Prepare the interview">%}**/interview** builds a prep pack from the exact posting and the documents the interviewer read, researches the company and the interviewers, matches likely questions to your STAR stories (situation, task, action, result) and offers a mock interview.{%</step>%}

{{<subhead text="More commands" icon="lightbulb" />}}

{%<row key="/html-report">%}An offline dashboard of all your applications.{%</row>%}

{%<row key="/upskill">%}Your skill gaps across the jobs you track, with a learning plan.{%</row>%}

{%<row key="/gmail-sync">%}Reads interview invites and rejections in Gmail and proposes tracker updates for you to approve.{%</row>%}

{%<row key="/reset">%}Wipes your profile data so you can start over.{%</row>%}

{%<note>%}Source: the AI Job Search README and command files, checked 23 October 2026. [**AI Job Search on GitHub**](https://github.com/ai-job-search-dev/ai-job-search){%</note>%}
