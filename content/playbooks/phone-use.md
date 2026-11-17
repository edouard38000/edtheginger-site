+++
title = "How to let Claude control your iPhone with Phone Use"
date = 2026-11-16
description = "Set up Phone Use so Claude can open apps and tap through them on your iPhone, with the fixes for what went wrong on camera."

[extra]
cover = "covers/phone-use-playbook.jpg"
+++
I saw a reel saying Claude can now use an iPhone by itself, so I tested it. I asked Claude to put sushi for two in my Uber Eats basket, and it did, on my real iPhone, with no cable. This is every step I followed, including the things that went wrong, written for someone who has never done anything like this.

{{<subhead text="What you need" icon="check" level={2} />}}

{%<row key="A spare PC">%}An old laptop or desktop you no longer use, with Wi-Fi and Bluetooth. It becomes a Linux PC, and everything on it gets erased. Use a PC, not a Mac: on a Mac, Phone Use needs an extra part.{%</row>%}

{%<row key="A USB stick">%}8 GB or more. It gets erased too.{%</row>%}

{%<row key="Your everyday computer">%}Only to prepare the USB stick. Mac or Windows, both work.{%</row>%}

{%<row key="An iPhone">%}A normal one. No jailbreak and no cable.{%</row>%}

{%<row key="A paid Claude plan">%}Claude Code is the tool that does the work, and the free plan does not include it. My link below gives you a free week, while my guest passes last.{%</row>%}

{%<row key="One Wi-Fi network">%}The PC and the iPhone have to be on the same home Wi-Fi.{%</row>%}

{%<row key="About an hour">%}My setup took 41 minutes once the PC was a Linux PC. After that, each task is one sentence.{%</row>%}

{{<buttons texts={["Free week of Claude Code", "Phone Use on GitHub"]} urls={["https://claude.ai/referral/xylnBopBlw", "https://github.com/lxe/phoneuse"]} />}}

{{<subhead text="The five parts" level={2} />}}

1. [**Turn the spare PC into a Linux PC.**](#part-1) Skip this if it already runs Linux.
2. [**Let Claude install Phone Use on the PC.**](#part-2) One sentence, Claude does the rest.
3. [**Pair the iPhone.**](#part-3) The app, a QR code and one firewall fix.
4. [**Connect the screen and the taps.**](#part-4) Screen Mirroring, Bluetooth and two iPhone settings.
5. [**Run it.**](#part-5) Ask for what you want, tap Allow, and watch.

{%<part n={1} id="part-1" title="Turn the spare PC into a Linux PC">%}
Phone Use works best on Linux, a free system that replaces Windows. I use a version called Omarchy, and it already has Claude Code inside. My PC was already running it, so I did not film this part: these steps come from the Omarchy manual.
{%</part>%}

{{<label text="THIS ERASES THE WHOLE PC" />}}

Installing Omarchy wipes the PC. Copy off anything you want to keep first. Only do this on a computer you do not need for anything else.

{{<subhead text="On your everyday computer" icon="lightbulb" />}}

{%<step n={1} title="Download Omarchy">%}Go to omarchy.org and download the file it offers. It is big and ends in .iso.{%</step>%}

{%<step n={2} title="Download balenaEtcher">%}It is a free app that copies that file onto a USB stick the right way. Dragging the file onto the stick does not work.{%</step>%}

{%<step n={3} title="Make the USB stick">%}Plug in the stick, open balenaEtcher, choose the Omarchy file, choose the stick, and click Flash. Wait until it says it is done.{%</step>%}

{{<buttons texts={["Omarchy", "balenaEtcher", "The Omarchy manual"]} urls={["https://omarchy.org/", "https://etcher.balena.io/", "https://learn.omacom.io/2/the-omarchy-manual/50/getting-started"]} />}}

{{<subhead text="On the spare PC" icon="lightbulb" />}}

{%<step n={1} title="Plug in the stick and a keyboard with a cable">%}A Bluetooth keyboard does not work during the installation.{%</step>%}

{%<step n={2} title="Turn off Secure Boot">%}Switch the PC on and keep pressing the key that opens its setup screen (often F2, F10, F12 or Delete; it depends on the brand, so search for your model and "BIOS key"). Find Secure Boot and turn it off. Omarchy cannot install with it on.{%</step>%}

{%<step n={3} title="Start from the stick">%}In the same setup screen, or with the boot menu key, choose the USB stick as the thing to start from.{%</step>%}

{%<step n={4} title="Answer the questions">%}Your name, a password, your Wi-Fi. Write the password down: you need it every time the PC starts, and again in [part 3](#part-3).{%</step>%}

{%<step n={5} title="Pick the drive and wait">%}Choose the PC's drive and confirm. The installation takes 2 to 10 minutes. Then the PC restarts as a Linux PC.{%</step>%}

{%<note>%}Stuck? The Omarchy manual has a help channel, and searching your PC model with "install Linux from USB" usually shows the exact keys.{%</note>%}

{%<part n={2} id="part-2" title="Let Claude install Phone Use">%}
You don't install anything by hand. You type one sentence to Claude Code on the Linux PC, and it downloads Phone Use, installs it and connects itself to it.
{%</part>%}

{%<step n={1} title="Open the terminal">%}The terminal is a window where you type instead of click. On Omarchy, hold the Super key (the Windows key) and press Enter.{%</step>%}

{%<step n={2} title="Start Claude Code">%}Type **claude** and press Enter. The first time, it opens a web page to log in to your Claude account. Log in and come back.{%</step>%}

{%<step n={3} title="Paste the first prompt below">%}Claude asks for permission before it runs things. Read what it asks and say yes.{%</step>%}

{%<step n={4} title='Wait for "installed and connected"'>%}Mine took about a minute. Claude then tells you what is left to do on the phone.{%</step>%}

{%<step n={5} title="Paste the second prompt">%}A Phone Use window opens on the PC. It shows a QR code, which you scan in the next part.{%</step>%}

{%<prompt label="PROMPT 1" dest="paste in Claude Code" logos="claude">%}
Install Phone Use, github.com/lxe/phoneuse, on this machine and connect it to Claude Code.
{%</prompt>%}

{%<prompt label="PROMPT 2" dest="paste in Claude Code" logos="claude">%}
Open Phone Use so I can pair my iPhone.
{%</prompt>%}

{{<step_image src="playbooks/phone-use/pc_install_prompt.jpg" label="WHAT IT LOOKED LIKE FOR ME" alt="The Claude Code message box with the prompt typed in: Install PhoneUse on this machine and connect it to Claude Code." />}}

{{<subhead text="How I did it" icon="lightbulb" />}}

I stayed on my Mac. In the Claude desktop app I chose my Linux PC as the computer the session runs on, so I typed on the Mac and the work happened on the PC. That needs a remote connection (SSH) set up between the two. Typing straight on the Linux PC, as described above, gives the same result and is simpler.

{%<part n={3} id="part-3" title="Pair the iPhone">%}
Pairing tells the PC and the iPhone that they belong together. Check first that the iPhone is on the same Wi-Fi as the PC, not on mobile data or a guest network.
{%</part>%}

{{<shots srcs={["playbooks/phone-use/ip_appstore.jpg"]} labels={["THE RIGHT APP"]} alts={["The Phone Use app in the App Store, by Aleksey Smolenchuk, with a pink cloud holding a phone as its icon."]} />}}

{%<step n={1} title="Install the Phone Use app">%}On the iPhone, open the App Store and search for Phone Use. Take the one by Aleksey Smolenchuk, with the pink cloud holding a phone.{%</step>%}

{%<step n={2} title="Scan the QR code">%}Open the app and point the iPhone at the QR code in the Phone Use window on the PC.{%</step>%}

{%<step n={3} title="If the phone says it can't reach the PC">%}This happened to me. The PC's firewall, the part that blocks outside connections, was blocking the iPhone, and the app did not say so. Paste the prompt below.{%</step>%}

{%<step n={4} title="Type your PC password when asked">%}Claude prepares the fix but cannot type your password. It shows you where to type it. Type it, press Enter, then scan the QR code again.{%</step>%}

{%<prompt label="PROMPT" dest="paste in Claude Code" logos="claude">%}
On my phone it says it can't reach this computer. Check that Phone Use is running, and open what the firewall is blocking for it, only for my home network.
{%</prompt>%}

{{<subhead text="Three switches in the Phone Use window on the PC" icon="check" />}}

{%<row key="Use This PC's Bluetooth">%}On. This is how the taps reach the iPhone. It asks for your PC password once. While it is on, the PC cannot use a Bluetooth keyboard or mouse of its own.{%</row>%}

{%<row key="Agent control">%}Ask every time. The iPhone then asks you before Claude takes control. I would not choose Always.{%</row>%}

{%<row key="Keep Awake">%}On. Claude cannot see or use the iPhone once its screen sleeps.{%</row>%}

{{<buttons texts={["Phone Use on the App Store"]} urls={["https://apps.apple.com/us/app/phone-use/id6832337610"]} />}}

{%<part n={4} id="part-4" title="Connect the screen and the taps">%}
The PC sees the iPhone through Screen Mirroring and taps on it through Bluetooth, as if it were a mouse and a keyboard. Keep the iPhone unlocked while you do this.
{%</part>%}

{{<shots srcs={["playbooks/phone-use/ip_mirroring.jpg", "playbooks/phone-use/ip_bluetooth.jpg"]} labels={["01 SCREEN MIRRORING", "02 BLUETOOTH"]} alts={["The iPhone Control Center, with the Screen Mirroring button, two rectangles, highlighted.", "The iPhone Bluetooth settings, with Phone Use connected."]} />}}

{%<step n={1} title="Start Screen Mirroring">%}Swipe down from the top right corner, tap the two rectangles, and choose "Phone Use on" followed by your PC's name.{%</step>%}

{%<step n={2} title="Connect Bluetooth">%}Settings, Bluetooth, tap Phone Use and accept. It has to say Connected.{%</step>%}

{%<step n={3} title="Turn on AssistiveTouch">%}Settings, Accessibility, Touch, AssistiveTouch: on. On the same page, Dwell Control: off. A round button appears on screen. Without it Claude can type but not tap.{%</step>%}

{%<step n={4} title="Make the pointer orange">%}Settings, Accessibility, Pointer Control, Colour: Orange.{%</step>%}

{{<subhead text="If Bluetooth will not stay connected" icon="exclamation" />}}

Mine connected and dropped several times. What fixed it: in Settings, Bluetooth, tap the (i) next to Phone Use and choose Forget This Device, pair again, then tap Phone Use once more to connect. Pairing can stop Screen Mirroring, so start it again after.

{%<prompt label="PROMPT: CHECK EVERYTHING" dest="paste in Claude Code" logos="claude">%}
Check if my iPhone is paired, and if screen mirroring and Bluetooth are connected now.
{%</prompt>%}

{%<prompt label="PROMPT: FIX THE SWIPES" dest="paste in Claude Code" logos="claude">%}
Calibrate the pointer on my phone so swipes and scrolling work properly.
{%</prompt>%}

I ran the second one only after my sushi test, and I should have done it before. Without it Claude's swipes moved the screen by almost nothing. With the orange pointer the calibration worked first try, and scrolling was normal afterwards.

{%<part n={5} id="part-5" title="Run it">%}
Now ask Claude Code for what you want, the way you would ask a person. Say which app, and say where it has to stop.
{%</part>%}

{%<prompt label="PROMPT" dest="paste in Claude Code" logos="claude" example_label="Mine:" example="I'm really hungry tonight. Can you please order sushi on Uber Eats for me and my girlfriend using my phone? Add everything to the basket, but don't click on checkout. I will handle that part myself.">%}
Using my phone, [what you want done] on [the app]. [Where it must stop, for example: add everything to the basket, but don't click on checkout]. I will handle that part myself.
{%</prompt>%}

{{<shots srcs={["playbooks/phone-use/ph_B12_agent_requests.jpg", "playbooks/phone-use/ph_B18_open_restaurant.jpg", "playbooks/phone-use/ph_B23_add_to_cart.jpg"]} labels={["01 TAP ALLOW", "02 IT PICKS A RESTAURANT", "03 IT ADDS A DISH"]} alts={["The iPhone saying Agent taking control, Opening Uber Eats, with Cancel and Allow now buttons.", "Uber Eats open on the Maki Maki sushi restaurant, picked by Claude.", "The Crispy (california roll) dish in Uber Eats, about to be added to the basket."]} />}}

{%<step n={1} title="Send the prompt, then tap Allow">%}The iPhone shows that an agent wants control, and what for. Tap Allow, or Cancel to stop it.{%</step>%}

{%<step n={2} title="Leave the phone alone">%}Claude opens the app, reads the screen and taps by itself. Mine opened Uber Eats, tapped Sushi, picked a restaurant, translated the menu and added three dishes to the basket.{%</step>%}

{%<step n={3} title="Check the basket and pay yourself">%}It stopped before checkout, as I asked, and told me what it chose and why. Look at it before you order.{%</step>%}

{{<subhead text="Write your own" icon="lightbulb" />}}

{%<row key="Name the app">%}Uber Eats, the App Store, Safari. Claude finds it on the phone and opens it.{%</row>%}

{%<row key="Say where it stops">%}In the basket, before sending, before paying. It respected that on my run.{%</row>%}

{%<row key="One task at a time">%}One clear task per message.{%</row>%}

{%<part n={6} id="the-result" title="The result" label="">%}
What I got from one sentence, with the phone lying on the desk and nothing plugged into it.
{%</part>%}

{{<shots srcs={["playbooks/phone-use/ph_B26_bogo.jpg"]} labels={["THE BASKET IT FILLED"]} alts={["The Uber Eats basket at Maki Maki: Crispy (california roll) with a Buy 1, get 1 free offer, Crazy salmon (california roll) and Chicken gyoza, saving €9.80 with offers."]} />}}

{{<subhead text="My verdict: REAL" />}}

It works, on a normal iPhone, with nothing plugged in. Two crispy rolls, a salmon roll and chicken gyoza were in my Uber Eats basket for €28.45, with a buy 1, get 1 free offer on the crispy rolls. It stopped before checkout.

{{<step_image src="playbooks/phone-use/timer_done.jpg" width={924} label="HOW LONG IT TOOK" alt="Claude Code's finished task list: Connect to the iPhone, Open Uber Eats and find sushi, Add sushi for two to basket, Verify basket, stop before checkout. 5m 57s, 2.3k tokens." />}}

It is slow: 5 minutes 57 seconds to fill a basket. I had not calibrated the pointer yet, so its swipes fell very short and it only looked at the first few restaurants.

{{<subhead text="What went wrong for me, and the fix" icon="exclamation" />}}

{%<row key="Phone can't reach the PC">%}The PC's firewall. The prompt in [part 3](#part-3) fixes it.{%</row>%}

{%<row key="Bluetooth keeps dropping">%}Forget the device on the iPhone, pair again, tap Phone Use once more.{%</row>%}

{%<row key="Claude types but can't tap">%}AssistiveTouch was off. Turn it on, Dwell Control off.{%</row>%}

{%<row key="Swipes barely move">%}Orange pointer, then the calibration prompt in [part 4](#part-4).{%</row>%}

{%<row key="A physical button">%}Claude cannot press the side button. To install an app, you double-click it yourself.{%</row>%}

{{<subhead text="What to watch out for" icon="exclamation" />}}

{%<row key="Money">%}Always tell it to stop before checkout or payment, and do that step yourself.{%</row>%}

{%<row key="Your accounts">%}It acts as you in every app it opens. Don't ask it to post or send messages unless you are fine with the result.{%</row>%}

{%<row key="Alpha software">%}Phone Use is new and its author says so. Expect bugs like the ones above.{%</row>%}

{%<note>%}This playbook comes from Real or BS, my series where I test what viral AI videos claim. Tag me in the comments of a video you want me to test next.{%</note>%}
