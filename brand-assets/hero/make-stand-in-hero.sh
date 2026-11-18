#!/usr/bin/env bash
# Makes the stand-in Hero of the home page from Edouard's relit portrait, with ffmpeg and cwebp:
#   static/home/hero.mp4         about 6 s, 720 x 1280, 30 fps, H.264, no audio track, fast-start
#   static/home/hero-poster.webp its first frame, shown before the video plays and with "reduce motion"
#
# Hard cuts between black-and-white, negative and orange grades of the portrait, each shot with its
# own crop and a slow zoom in or out, so the cuts feel like different shots. The cuts start 0.2 s
# apart, speed up to 0.1 s, then slow down to 0.3 s before the loop starts again.
#
# Shell and not Rust: it only chains two ffmpeg calls and builds one filter graph, and the repo's
# Rust crate is the check harness, not a place for one-off asset tools.
#
# Run from the repo root:  brand-assets/hero/make-stand-in-hero.sh
# The real Hero (made in the editing project with Higgsfield) replaces the two files; see
# [extra.hero] in content/_index.md.
set -euo pipefail

source_image="brand-assets/portrait/portrait-relit-ai.png" # 1824 x 2288
video="static/home/hero.mp4"
poster="static/home/hero-poster.webp"

# Length of each shot in frames at 30 fps: 0.2 s, speeding up to 0.1 s, slowing down to 0.3 s.
frames=(6 6 6 6 5 5 4 4 3 3 3 3 3 3 3 3 3 3 4 4 5 5 6 6 7 7 8 8 9 9 9 9 9 9)

# Crops of the portrait, all 9:16, as "width:height:x:y" in the source's pixels.
crops=(
  "1287:2288:250:0"    # the whole portrait
  "540:960:625:520"    # the face
  "450:800:670:640"    # the eyes
  "810:1440:490:250"   # head and shoulders
  "675:1200:1000:300"  # off to the right, the face on the left edge
  "675:1200:300:500"   # off to the left, the face on the right edge
  "608:1080:590:950"   # mouth and collar
)

# Grades: black-and-white, negative, orange. All stay dark at the edges, where the white text sits:
# the negative is pulled down and vignetted (a plain negative of a dark photo is white), the orange
# maps black to black and the light on the face to the brand orange.
grades=(
  "format=gray,eq=contrast=1.45:brightness=0.03,format=rgb24"
  "format=gray,negate,curves=all='0/0 0.5/0.22 1/0.72',vignette=angle=PI/3.2,format=rgb24"
  "format=gray,eq=contrast=1.35:brightness=0.02,format=rgb24,colorchannelmixer=rr=1:rg=0:rb=0:gr=0.36:gg=0:gb=0:br=0.04:bg=0:bb=0"
)

count=${#frames[@]}
graph="[0:v]format=rgb24,split=${count}"
for ((i = 0; i < count; i++)); do graph+="[s${i}]"; done
graph+=";"
for ((i = 0; i < count; i++)); do
  crop=${crops[$(((i * 3) % ${#crops[@]}))]}
  grade=${grades[$((i % ${#grades[@]}))]}
  d=${frames[$i]}
  # Every other shot zooms in, the others zoom out: about 0.6 % per frame.
  if ((i % 2 == 0)); then zoom="1+0.006*on"; else zoom="1.12-0.006*on"; fi
  graph+="[s${i}]crop=${crop},scale=1440:2560:flags=lanczos,${grade},"
  graph+="zoompan=z='${zoom}':x='iw/2-iw/zoom/2':y='ih/2-ih/zoom/2':d=${d}:s=720x1280:fps=30,"
  graph+="setsar=1,setpts=PTS-STARTPTS[v${i}];"
done
for ((i = 0; i < count; i++)); do graph+="[v${i}]"; done
graph+="concat=n=${count}:v=1:a=0,format=yuv420p[out]"

ffmpeg -y -v error -i "$source_image" -filter_complex "$graph" -map "[out]" -an \
  -c:v libx264 -preset veryslow -crf 28 -profile:v high -pix_fmt yuv420p \
  -movflags +faststart "$video"

# The poster: the first frame, as WebP (cwebp, from Homebrew's webp package).
first_frame=$(mktemp -t hero-poster).png
ffmpeg -y -v error -i "$video" -frames:v 1 "$first_frame"
cwebp -quiet -q 78 "$first_frame" -o "$poster"
rm "$first_frame"

ls -l "$video" "$poster"
