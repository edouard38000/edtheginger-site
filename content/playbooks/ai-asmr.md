+++
title = "AI ASMR: from prompt to post"
date = 2026-10-06
description = "Make an AI ASMR video, from the first prompt to the finished post."

[extra]
cover = "covers/ai-asmr-playbook.jpg"
+++
Create one short video with several shots, add matching sound, edit it and schedule it. This example uses a 10-second clip. No n8n template or extra skill is needed.

{{<subhead text="The six parts" level={2} />}}

1. [**Pick your ASMR concept.**](#part-1)
2. [**Generate one multi-shot clip.**](#part-2)
3. [**Add sound to the whole clip.**](#part-3)
4. [**Edit the finished clip.**](#part-4)
5. [**Connect your social accounts.**](#part-5)
6. [**Prepare captions and schedule.**](#part-6)

{%<part n={1} id="part-1" title="Pick your ASMR concept">%}
{%</part>%}

{{<shots srcs={["playbooks/ai-asmr/concept_strawberries.jpg", "playbooks/ai-asmr/concept_mango.jpg", "playbooks/ai-asmr/concept_grapes.jpg", "playbooks/ai-asmr/concept_honeycomb.jpg"]} labels={["GLASS STRAWBERRIES", "CRYSTAL MANGO", "JEWEL GRAPES", "GLASS HONEYCOMB"]} alts={["Two strawberries made of red glass, one with a bite taken out of it.", "Half a mango made of amber crystal, cut into cubes.", "A bunch of grapes made of purple faceted glass, one of them bitten.", "A piece of honeycomb made of golden glass, with broken shards beside it."]} />}}

{%<step n={1} title="Pick your concept">%}Choose glass strawberries, crystal mango, jewel grapes or glass honeycomb.{%</step>%}

{%<step n={2} title="Open ChatGPT or Claude">%}Copy the full prompt below into your chat. Replace the placeholders with your concept and audience, then send. Example: glass strawberries; crunchy ASMR fans.{%</step>%}

{%<step n={3} title="Save both outputs">%}The chat should return one VIDEO PROMPT and one AUDIO PROMPT. Keep both for the next two steps.{%</step>%}

{{<buttons texts={["ChatGPT", "Claude"]} urls={["https://chatgpt.com/", "https://claude.ai/"]} />}}

{%<prompt label="PROMPT" dest="paste in ChatGPT or Claude" logos="chatgpt,claude">%}
Turn [SELECTED CONCEPT] into one 10-second vertical AI ASMR mukbang clip for [TARGET AUDIENCE].

Return two labeled blocks:

VIDEO PROMPT: Write one Seedance 1.0 prompt under 1,500 characters. Define one adult fictional character, imaginary glass-like food, setting and lighting. Describe three brief shots within 10 seconds: reveal the food, show one complete bite, then show the satisfying texture and chewing. Keep the character and food consistent. Use 9:16 close-ups, simple camera movement and clear cuts. Start immediately. No injuries, dialogue, captions or logos.

AUDIO PROMPT: Write one MMAudio prompt for the entire clip. Describe close-miked bites, cracks, crunches and chewing specific to the material. Match the actual visible actions, including across cuts. Quiet background, controlled volume; no speech or music.

Keep the shots achievable in 10 seconds. Do not output separate generation prompts for each shot.
{%</prompt>%}

{%<part n={2} id="part-2" title="Generate one multi-shot clip">%}
Use one video prompt for the whole clip. Several shots share the selected duration; three shots do not mean three separate 10-second videos.
{%</part>%}

{{<subhead text="Generate the video" />}}

{%<note>%}Seedance 1.0 · Lite / Pro / Pro Fast{%</note>%}

{{<shots srcs={["playbooks/ai-asmr/logo_fal.png", "playbooks/ai-asmr/logo_wavespeed.png", "playbooks/ai-asmr/logo_kie.png", "playbooks/ai-asmr/logo_replicate.png"]} labels={["FAL.AI", "WAVESPEED", "KIE.AI", "REPLICATE"]} alts={["The fal logo.", "The WaveSpeed logo.", "The Kie.ai logo.", "The Replicate logo."]} />}}

{%<step n={1} title="Choose one provider">%}Open a link below, sign in and add credits. WaveSpeed Lite 480p is the budget example.{%</step>%}

{%<step n={2} title="Set the generation">%}Choose text-to-video, 9:16 and 10 seconds. Use 480p for the budget test; choose 720p or 1080p where supported for a sharper result.{%</step>%}

{%<step n={3} title="Paste and download">%}Paste the VIDEO PROMPT from [part 1](#part-1) once. Generate, watch the entire result and save it as asmr-video.mp4.{%</step>%}

{{<subhead text="Choose the model listed for your provider" icon="check" />}}

{%<row key="fal.ai">%}Seedance 1.0 Pro Fast{%</row>%}

{%<row key="WaveSpeed">%}Seedance V1 Lite T2V 480p{%</row>%}

{%<row key="Kie.ai">%}V1 Pro Text To Video for multi-shot{%</row>%}

{%<row key="Replicate">%}bytedance/seedance-1-lite{%</row>%}

{{<buttons texts={["fal.ai", "WaveSpeed", "Kie.ai", "Replicate"]} urls={["https://fal.ai/models/fal-ai/bytedance/seedance/v1/pro/fast/text-to-video", "https://wavespeed.ai/models/bytedance/seedance-v1-lite-t2v-480p", "https://kie.ai/bytedance/seedance-v1", "https://replicate.com/bytedance/seedance-1-lite"]} />}}

{%<note>%}fal has retired its Lite endpoint and redirects to Pro Fast. Kie documents multi-shot support for Pro. Follow the linked provider page; variants and prices differ.{%</note>%}

{{<subhead text="Budget example: one 10-second generation" icon="lightbulb" />}}

{%<row key="WaveSpeed Lite video at 480p">%}$0.16{%</row>%}

{%<row key="WaveSpeed MMAudio V2 sound">%}$0.01{%</row>%}

{%<row key="Video + sound generation only">%}about $0.17{%</row>%}

{%<note>%}Published rate estimate, checked 6 October 2026. Excludes chat, editor and scheduler plans, credit top-ups, retries and higher-resolution output. Check the live generation quote before paying.{%</note>%}

{{<buttons texts={["Video pricing", "Audio pricing", "Seedance multi-shot overview"]} urls={["https://wavespeed.ai/models/bytedance/seedance-v1-lite-t2v-480p", "https://wavespeed.ai/models/wavespeed-ai/mmaudio-v2", "https://seed.bytedance.com/en/seedance"]} />}}

{%<part n={3} id="part-3" title="Add sound to the whole clip">%}
{%</part>%}

{{<subhead text="Generate the sound" />}}

{%<note>%}MMAudio{%</note>%}

{{<shots srcs={["playbooks/ai-asmr/logo_fal.png", "playbooks/ai-asmr/logo_wavespeed.png", "playbooks/ai-asmr/logo_replicate.png", "playbooks/ai-asmr/logo_eachlabs.png"]} labels={["FAL.AI", "WAVESPEED", "REPLICATE", "EACHLABS"]} alts={["The fal logo.", "The WaveSpeed logo.", "The Replicate logo.", "The Eachlabs logo."]} />}}

{%<step n={1} title="Choose MMAudio">%}Open one of the four providers below. fal and WaveSpeed offer MMAudio V2; the other listings are MMAudio.{%</step>%}

{%<step n={2} title="Upload your video">%}Upload asmr-video.mp4. Paste the AUDIO PROMPT from [part 1](#part-1). Match the sound duration to the actual clip, usually 10 seconds here.{%</step>%}

{%<step n={3} title="Listen and save">%}Check every bite and cut with headphones. Download the video with sound as asmr-sound.mp4, or keep the separate audio beside its video.{%</step>%}

{{<buttons texts={["fal.ai", "WaveSpeed", "Replicate", "Eachlabs"]} urls={["https://fal.ai/models/fal-ai/mmaudio-v2", "https://wavespeed.ai/models/wavespeed-ai/mmaudio-v2", "https://replicate.com/zsxkib/mmaudio", "https://www.eachlabs.ai/meta/mm-audio/mmaudio"]} />}}

Use your generated AUDIO PROMPT first. This reusable version is useful when you need to revise the sound after seeing the actual clip.

{%<prompt label="PROMPT" dest="paste in MMAudio" logos="">%}
Close-miked ASMR of [FOOD OR MATERIAL] being [VISIBLE EATING ACTION].

[SOUND TEXTURES FOR THE CLIP].

Match each bite, crack, crunch, and chewing sound to the corresponding movement in the uploaded video. Follow the actual action timing across all shots, with delicate texture sounds between stronger crunches.

Use a quiet background, clear detail, and controlled volume. No music, speech, narration, unrelated effects, harsh distortion, or exaggerated reverb.
{%</prompt>%}

**If the sound misses a bite:** describe the visible action more precisely and rerun the audio only. If your download contains audio alone, align it to the video start in the editor and keep the tracks linked.

{%<part n={4} id="part-4" title="Edit the finished clip">%}
{%</part>%}

{{<subhead text="Edit your video" />}}

{%<note>%}AI video editors{%</note>%}

{{<shots srcs={["playbooks/ai-asmr/logo_chatcut.png", "playbooks/ai-asmr/logo_descript.png", "playbooks/ai-asmr/logo_kapwing.png", "playbooks/ai-asmr/logo_veed.png"]} labels={["CHATCUT", "DESCRIPT", "KAPWING", "VEED"]} alts={["The ChatCut logo.", "The Descript logo.", "The Kapwing logo.", "The VEED logo."]} />}}

{%<step n={1} title="Import your clip">%}Open one editor below and upload asmr-sound.mp4. If sound is separate, import both files and align their starts.{%</step>%}

{%<step n={2} title="Open its assistant">%}Use ChatCut chat, Descript Underlord, Kapwing’s video assistant or VEED’s assistant. Paste the prompt below and identify the uploaded clip.{%</step>%}

{%<step n={3} title="Review and export">%}Check the cuts and audio sync. Make any unsupported adjustment in the editor, then export the final MP4.{%</step>%}

{{<buttons texts={["ChatCut", "Descript", "Kapwing", "VEED"]} urls={["https://chatcut.io", "https://www.descript.com/underlord", "https://www.kapwing.com/ai/video-assistant", "https://www.veed.io/tools/video-gpt/ai-video-assistant"]} />}}

{%<prompt label="PROMPT" dest="paste in your AI editor" logos="">%}
Edit [VIDEO CLIP] into a polished vertical 9:16 ASMR video.

Keep the clip’s audio linked and synchronized when trimming. Remove black frames, frozen endings and unnecessary waiting before or after the action. Preserve complete bites, satisfying crunches and quiet texture sounds.

Use clean cuts and consistent volume. Prevent clipping and abrupt audio clicks. Do not add music, voiceover, captions, transitions or generated footage. Keep the original playback speed.

Keep the natural duration without padding. Preserve the source resolution and export an MP4.

Flag any remaining visual artifacts or synchronization problems for review.
{%</prompt>%}

**Final playback check:** watch with headphones. Keep the complete bite, remove dead time and check for distorted faces, broken food geometry, black frames or an abrupt ending. Export at the source resolution; enlarging 480p does not restore detail.

{%<part n={5} id="part-5" title="Connect your social accounts">%}
{%</part>%}

{{<subhead text="Schedule your video" />}}

{%<note>%}Connect your social accounts{%</note>%}

{{<shots srcs={["playbooks/ai-asmr/logo_postiz.png", "playbooks/ai-asmr/logo_uploadpost.png", "playbooks/ai-asmr/logo_buffer.png", "playbooks/ai-asmr/logo_ayrshare.png"]} labels={["POSTIZ", "UPLOAD-POST", "BUFFER", "AYRSHARE"]} alts={["The Postiz logo.", "The Upload-Post logo.", "The Buffer logo.", "The Ayrshare logo."]} />}}

{%<step n={1} title="Pick one scheduler">%}Postiz is the example here. Create an account and connect the social profiles you want to use through its dashboard.{%</step>%}

{%<step n={2} title="Connect your AI agent">%}Add the official Postiz connector in ChatGPT or Claude where available, or follow its MCP setup guide below. Sign in to your Postiz account.{%</step>%}

{%<step n={3} title="Check the connection">%}Ask: “List my connected social accounts.” Confirm the returned profile names before giving the agent your video.{%</step>%}

{{<buttons texts={["Postiz", "Upload-Post", "Buffer", "Ayrshare"]} urls={["https://postiz.com/mcp", "https://docs.upload-post.com/guides/mcp-server-integration/", "https://buffer.com/mcp", "https://www.ayrshare.com/docs/additional/mcp-action-server"]} />}}

**MCP** is a connection that lets your assistant use an app. A prompt alone cannot connect accounts or schedule posts. Use one supported connector; you do not need all four services.

{{<subhead text="Connection guides" icon="check" />}}

{%<row key="Postiz">%}[Official connector or hosted MCP; browser sign-in.](https://postiz.com/mcp){%</row>%}

{%<row key="Upload-Post">%}[Hosted MCP; browser sign-in or API-key setup.](https://docs.upload-post.com/guides/mcp-server-integration/){%</row>%}

{%<row key="Buffer">%}[Use its integration guide for your AI tool.](https://buffer.com/mcp){%</row>%}

{%<row key="Ayrshare">%}[Action MCP with an API key; not the docs-only server.](https://www.ayrshare.com/docs/additional/mcp-action-server){%</row>%}

If a connector requires an API key, enter it in the connector’s secure settings. If your agent cannot upload local files, use the scheduler’s upload area and provide the resulting accessible media link.

{%<part n={6} id="part-6" title="Prepare captions and schedule">%}
{%</part>%}

{{<subhead text="From video to socials" />}}

{%<row key="01 Finished video">%}Give your agent the final MP4.{%</row>%}

{%<row key="02 Prepare captions">%}Write a caption for each platform.{%</row>%}

{%<row key="03 Review + approve">%}Check captions, accounts and times.{%</row>%}

{%<row key="04 Schedule the posts">%}Use your connected publishing app.{%</row>%}

{%<note>%}Confirm each post in the scheduled queue.{%</note>%}

{%<step n={1} title="Give the agent the video">%}Attach your final MP4 or provide a supported media URL. Replace every placeholder below, including the time zone.{%</step>%}

{%<step n={2} title="Review the plan">%}Check the exact accounts, captions, date and time. Keep the AI-generated disclosure. Approve only the version you want posted.{%</step>%}

{%<step n={3} title="Confirm the queue">%}Check the scheduler for one post per destination. A message saying “done” is not enough; verify its status and scheduled time.{%</step>%}

{%<prompt label="PROMPT" dest="paste in your connected agent" logos="">%}
Prepare [FINISHED VIDEO FILE OR URL] for publication using [CONNECTED PUBLISHING SERVICE].

Video concept: [CONCEPT].
Destination accounts: [PLATFORMS AND ACCOUNTS].
Schedule: [DATE, TIME, AND TIME ZONE].

Write a short caption for each platform with relevant hashtags. Describe the video as AI-generated ASMR. Avoid earnings promises, invented claims, or presenting the footage as real.

Check each platform’s upload requirements and apply supported AI-content disclosures.

Show me the captions, destination accounts, and posting times for approval. After approval, schedule each post once.

Return each platform’s confirmed scheduled time and post ID or link. Report failed or unsupported actions clearly. Do not claim a post is scheduled without confirmation from the publishing service.
{%</prompt>%}

**Keep your files:** save the video prompt, audio prompt, final MP4 and editing project together. If a post fails, check its status before retrying so you do not create duplicates.
