//! Makes the Poster's lit portrait from `brand-assets/portrait/portrait-cutout.png`.
//!
//! The light is a beam through a slit: the strip between two parallel diagonal lines. The beam
//! is read from `[extra.beam]` in `content/_index.md`, the same numbers the home template uses to
//! draw the two white diagonal lines, so the lit strip and the lines always match. Every number
//! there is in pixels of the cut-out (1350 x 1800).
//!
//! Writes:
//! - `static/home/portrait-lit.webp`: the portrait in shadow, lit between the lines, with a warm
//!   rim light on the edges facing the background glow and the cut-out's dark fringe removed;
//! - `static/home/grain.png`: a small tile of noise the stylesheet lays over the background
//!   gradients so they never show bands;
//! - `scripts/light-portrait/beam-used.toml`: the beam the image was made with and the image's
//!   size, which `tests/home.rs` compares with `content/_index.md` and the published file.
//!
//! Run from anywhere: `cargo run --release --manifest-path scripts/light-portrait/Cargo.toml`.

use std::fs;
use std::path::{Path, PathBuf};

use image::{GrayImage, Luma, Rgba, RgbaImage};
use serde::Deserialize;

/// The beam, in pixels of the cut-out.
#[derive(Debug, Deserialize)]
struct Beam {
    /// A point on the middle line of the beam (between the eyes).
    x: f32,
    y: f32,
    /// How far the lines drop per pixel to the right (upper left to lower right).
    slope: f32,
    /// The vertical distance between the two lines.
    width: f32,
    /// How many pixels the light takes to fade out at each line.
    soft: f32,
}

#[derive(Deserialize)]
struct FrontMatter {
    extra: Extra,
}

#[derive(Deserialize)]
struct Extra {
    beam: Beam,
}

const QUALITY: f32 = 84.0;

fn main() {
    let root = repo_root();
    let beam = read_beam(&root.join("content/_index.md"));
    let source = root.join("brand-assets/portrait/portrait-cutout.png");
    let cutout = image::open(&source)
        .unwrap_or_else(|e| panic!("open {}: {e}", source.display()))
        .to_rgba8();

    let alpha = clean_alpha(&cutout);
    let colours = decontaminate(&cutout, &alpha);
    let lit = light(&colours, &alpha, &beam);

    let out = root.join("static/home/portrait-lit.webp");
    let webp = webp::Encoder::from_rgba(lit.as_raw(), lit.width(), lit.height()).encode(QUALITY);
    fs::write(&out, &*webp).unwrap_or_else(|e| panic!("write {}: {e}", out.display()));

    let grain = root.join("static/home/grain.png");
    noise_tile(160).save(&grain).expect("write the grain tile");

    let used = root.join("scripts/light-portrait/beam-used.toml");
    let record = format!(
        "# Written by scripts/light-portrait: the beam static/home/portrait-lit.webp was lit with.\n\
         # tests/home.rs checks it matches [extra.beam] in content/_index.md and the published file.\n\
         x = {}\ny = {}\nslope = {}\nwidth = {}\nsoft = {}\nimage_bytes = {}\n",
        beam.x,
        beam.y,
        beam.slope,
        beam.width,
        beam.soft,
        webp.len()
    );
    fs::write(&used, record).expect("write beam-used.toml");

    println!(
        "{} ({} x {}, {} KB), {}",
        out.display(),
        lit.width(),
        lit.height(),
        webp.len() / 1024,
        grain.display()
    );
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the repo root")
}

/// `[extra.beam]` from the TOML front matter of the home page.
fn read_beam(path: &Path) -> Beam {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let front = text
        .split("+++")
        .nth(1)
        .expect("content/_index.md has +++ front matter");
    let parsed: FrontMatter = toml::from_str(front).expect("[extra.beam] in content/_index.md");
    parsed.extra.beam
}

/// The cut-out's alpha without its dark fringe.
///
/// The flash left the wall's shadow beside the left ear and neck inside the cut-out: dark,
/// colourless pixels touching the transparent area. They are removed by growing the transparent
/// area into them (at most 22 px, only in that corner of the image). Then the whole edge is pulled
/// in by 2 px and softened, which drops the thin grey halo left elsewhere.
fn clean_alpha(cutout: &RgbaImage) -> GrayImage {
    let (w, h) = cutout.dimensions();
    let index = |x: u32, y: u32| (y * w + x) as usize;
    let in_corner = |x: u32, y: u32| {
        y as f32 >= h as f32 * 0.295 && (y as f32) < h as f32 * 0.47 && (x as f32) < w as f32 * 0.42
    };
    let shadow: Vec<bool> = cutout
        .enumerate_pixels()
        .map(|(x, y, p)| {
            let [r, g, b, _] = p.0.map(i32::from);
            let lum = (r + g + b) / 3;
            let sat = r.max(g).max(b) - r.min(g).min(b);
            lum < 95 && sat < 38 && in_corner(x, y)
        })
        .collect();
    let mut gone: Vec<bool> = cutout.pixels().map(|p| p.0[3] < 128).collect();
    for _ in 0..22 {
        let before = gone.clone();
        for y in 0..h {
            for x in 0..w {
                let i = index(x, y);
                if before[i] || !shadow[i] {
                    continue;
                }
                let touches = (x > 0 && before[index(x - 1, y)])
                    || (x + 1 < w && before[index(x + 1, y)])
                    || (y > 0 && before[index(x, y - 1)])
                    || (y + 1 < h && before[index(x, y + 1)]);
                gone[i] = touches;
            }
        }
    }
    let alpha = GrayImage::from_fn(w, h, |x, y| {
        Luma([if gone[index(x, y)] {
            0
        } else {
            cutout.get_pixel(x, y).0[3]
        }])
    });
    image::imageops::blur(&min_filter(&alpha, 2), 1.2)
}

/// The cut-out's edge pixels still carry some of the grey wall; they show as a pale outline once
/// the portrait is in shadow. Near the edge, each colour is pulled towards the colour of the
/// silhouette just inside it.
fn decontaminate(cutout: &RgbaImage, alpha: &GrayImage) -> RgbaImage {
    let (w, h) = cutout.dimensions();
    let inside = min_filter(alpha, 5);
    let weight = image::imageops::blur(&inside, 5.0);
    let channel = |c: usize| {
        let premultiplied = GrayImage::from_fn(w, h, |x, y| {
            let v = u32::from(cutout.get_pixel(x, y).0[c]) * u32::from(inside.get_pixel(x, y).0[0]);
            Luma([(v / 255) as u8])
        });
        image::imageops::blur(&premultiplied, 5.0)
    };
    let sums = [channel(0), channel(1), channel(2)];
    RgbaImage::from_fn(w, h, |x, y| {
        let own = cutout.get_pixel(x, y).0;
        let a = alpha.get_pixel(x, y).0[0];
        let core = f32::from(inside.get_pixel(x, y).0[0]) / 255.0;
        let wsum = f32::from(weight.get_pixel(x, y).0[0]);
        if a == 0 || core >= 1.0 || wsum < 8.0 {
            return Rgba([own[0], own[1], own[2], a]);
        }
        let mut out = [0u8; 4];
        for c in 0..3 {
            let estimate = (f32::from(sums[c].get_pixel(x, y).0[0]) * 255.0 / wsum).min(255.0);
            let mixed = f32::from(own[c]) * core + estimate * (1.0 - core);
            out[c] = mixed.round() as u8;
        }
        out[3] = a;
        Rgba(out)
    })
}

/// Each pixel becomes the smallest value within `radius` pixels (a square), in two passes.
fn min_filter(image: &GrayImage, radius: i64) -> GrayImage {
    let (w, h) = image.dimensions();
    let at = |img: &GrayImage, x: i64, y: i64| {
        let x = x.clamp(0, w as i64 - 1) as u32;
        let y = y.clamp(0, h as i64 - 1) as u32;
        img.get_pixel(x, y).0[0]
    };
    let rows = GrayImage::from_fn(w, h, |x, y| {
        let m = (-radius..=radius)
            .map(|d| at(image, x as i64 + d, y as i64))
            .min();
        Luma([m.unwrap_or(0)])
    });
    GrayImage::from_fn(w, h, |x, y| {
        let m = (-radius..=radius)
            .map(|d| at(&rows, x as i64, y as i64 + d))
            .min();
        Luma([m.unwrap_or(0)])
    })
}

/// 0 at `edge0`, 1 at `edge1`, smooth in between.
fn smoothstep(edge0: f32, edge1: f32, v: f32) -> f32 {
    let t = ((v - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// How much of the beam reaches a pixel: 1 between the two lines, 0 outside, fading over `soft`
/// pixels across each line.
fn in_beam(beam: &Beam, x: f32, y: f32) -> f32 {
    let middle = beam.y + beam.slope * (x - beam.x);
    // Distance across the beam, measured at right angles to the lines.
    let across = (y - middle).abs() / (1.0 + beam.slope * beam.slope).sqrt();
    let half = beam.width / 2.0 / (1.0 + beam.slope * beam.slope).sqrt();
    smoothstep(half + beam.soft, half - beam.soft, across)
}

/// The portrait in shadow, lit only between the lines, with a warm rim light.
fn light(cutout: &RgbaImage, alpha: &GrayImage, beam: &Beam) -> RgbaImage {
    // How far each pixel is inside the silhouette, for the rim light.
    let wide = image::imageops::blur(alpha, 3.5);
    RgbaImage::from_fn(cutout.width(), cutout.height(), |x, y| {
        let a = alpha.get_pixel(x, y).0[0];
        if a == 0 {
            return Rgba([0, 0, 0, 0]);
        }
        let [r, g, b, _] = cutout.get_pixel(x, y).0.map(|v| f32::from(v) / 255.0);
        let (fx, fy) = (x as f32, y as f32);

        // In the beam the light is strongest on the eyes and the bridge of the nose and fades
        // over the cheekbones and the hair.
        let dx = (fx - beam.x) / 230.0;
        let falloff = 0.55 + 0.45 * (-dx * dx).exp();
        let hx = (fx - beam.x) / 105.0;
        let hy = (fy - beam.y) / 55.0;
        let hot = 0.22 * (-(hx * hx + hy * hy)).exp();
        let light = in_beam(beam, fx, fy) * (falloff + hot);

        // Shadow: dark and warm, the face still just readable. Lit: warm, a little brighter.
        let shadow = [r * 0.30, g * 0.19, b * 0.14];
        let lit = [r * 1.18, g * 1.05, b * 0.92];
        let mut out = [0.0f32; 3];
        for i in 0..3 {
            out[i] = shadow[i] + (lit[i] - shadow[i]) * light;
        }

        // Rim light: the glow behind the head catches the edges of the hair and the tops of the
        // shoulders, the edges facing up and out; a thin line, fading away from the head.
        let (gx, gy) = gradient(&wide, x, y);
        let length = (gx * gx + gy * gy).sqrt();
        let facing_up = if length > 0.0 {
            (gy / length).max(0.0)
        } else {
            0.0
        };
        let edge =
            ((f32::from(a) - f32::from(wide.get_pixel(x, y).0[0])) / 255.0 * 2.5).clamp(0.0, 1.0);
        let rx = fx - beam.x;
        let ry = fy - (beam.y - 120.0);
        let near_head = (-(rx * rx + ry * ry) / (2.0 * 480.0 * 480.0)).exp();
        let rim = edge * near_head * (0.10 + 0.32 * facing_up);
        let warm = [1.0, 0.56, 0.20];
        for i in 0..3 {
            out[i] += warm[i] * rim;
        }

        let to_u8 = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
        Rgba([to_u8(out[0]), to_u8(out[1]), to_u8(out[2]), a])
    })
}

/// The slope of a grey image at a pixel, pointing towards higher values (into the silhouette).
fn gradient(image: &GrayImage, x: u32, y: u32) -> (f32, f32) {
    let (w, h) = image.dimensions();
    let at = |x: u32, y: u32| f32::from(image.get_pixel(x.min(w - 1), y.min(h - 1)).0[0]);
    let gx = at(x + 1, y) - at(x.saturating_sub(1), y);
    let gy = at(x, y + 1) - at(x, y.saturating_sub(1));
    (gx, gy)
}

/// A square tile of fine black and white specks with low alpha, the same every run.
fn noise_tile(size: u32) -> RgbaImage {
    let mut state: u32 = 0x9e37_79b9;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        state
    };
    RgbaImage::from_fn(size, size, |_, _| {
        let n = next();
        let v = if n & 1 == 0 { 0 } else { 255 };
        let a = ((n >> 8) % 13) as u8;
        Rgba([v, v, v, a])
    })
}
