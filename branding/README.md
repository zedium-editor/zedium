# Zedium — Brand Assets

Logo and icon assets for **Zedium**, a community fork of Zed.

The mark is a periodic-element tile carrying the symbol **Zm**, finished in brushed
gunmetal-blue — "zedium" reads like a metal, so the tile looks like one. It is
deliberately unlike Zed's stylised blue **Z**: a different object (a tile, not a letter)
in a darker, desaturated alloy tone.

## Contents

```
svg/                          Vector sources (edit these)
  zedium-icon.svg             Primary app icon — filled gunmetal tile
  zedium-favicon.svg          Same mark, for browser favicons
  zedium-icon-outline.svg     Hairline steel outline, for dark UI chrome
  zedium-icon-mono.svg        Single-colour (uses currentColor), Zm knocked out
  zedium-avatar.svg           GitHub org avatar (safe margin for circular crop)
  zedium-wordmark-dark.svg    Wordmark for dark grounds
  zedium-wordmark-light.svg   Wordmark for light grounds
  zedium-lockup-dark.svg      Icon + wordmark + tagline (README header), dark
  zedium-lockup-light.svg     Same, light

png/                          Rasterised, ready to use (font baked in)
  zedium-icon-16..1024.png    App-icon sizes
  zedium-avatar-256/512.png   Org avatar
  zedium-icon-outline-512.png
  zedium-wordmark-*.png        Transparent background
  zedium-lockup-*.png          Transparent background

favicon.ico                   Multi-size icon (16/32/48) for the web
zedium.icns                   macOS application icon
```

## Palette

| Token            | Hex        | Use                              |
|------------------|------------|----------------------------------|
| Gunmetal light   | `#6C8DB1`  | Tile gradient — top-left sheen   |
| Gunmetal mid     | `#47668E`  | Tile gradient — mid band         |
| Gunmetal deep    | `#101F35`  | Tile gradient — bottom shadow    |
| Steel (accent)   | `#6F9AD0`  | "ium" suffix / accents on dark   |
| Steel (accent)   | `#2E547E`  | "ium" suffix / accents on light  |
| Glyph            | `#EAF0F7`  | "Zm" knockout on the tile        |
| Ink (dark UI)    | `#0E141D`  | Dark ground                      |
| Off-white        | `#EDF0F4`  | Light ground                     |

## Typography

Wordmark and glyph are set in a **monospace** typeface (the native type of a code
editor). The PNGs were rasterised with the glyph baked in, so they are portable.
The **SVGs reference a system monospace font** (`ui-monospace, Menlo, monospace`) — if you
open them on a machine without a matching mono font the letter shapes will shift.
For a fully locked, font-independent vector, open an SVG in a vector editor and
**convert the text to outlines/paths**.

## Usage notes

- The icon's colours are **fixed** — do not recolour the mark to a viewer's light/dark theme.
- `zedium-icon-mono.svg` inherits `currentColor`; set `color:` on the parent when inlining.
- Keep clear space around the mark equal to the tile's corner radius.
- Do not stretch, rotate, or recolour into Zed's brand blue.

## App-icon replacement set (`app-icons/`)

Drop-in replacements at Zed's exact filenames and sizes. To rebrand a build, copy
straight over the originals:

| From `app-icons/` | To (in repo) |
|---|---|
| `app-icon.png`, `app-icon@2x.png` (512 / 1024) | `crates/zed/resources/` |
| `app-icon-{dev,nightly,preview}.png` + `@2x` | `crates/zed/resources/` |
| `windows/app-icon*.ico` (16→256 multi-res) | `crates/zed/resources/windows/` |
| `Document.icns` (1024) | `crates/zed/resources/` |
| `svg/zedium-logo.svg` (96×96) | replaces `assets/images/zed_logo.svg` (in-app logo) |

**Release-channel tints** (Zed differentiates channels by icon colour):

| Channel | Tone | Gradient top → bottom |
|---------|------|-----------------------|
| stable  | gunmetal blue    | `#6C8DB1` → `#101F35` |
| preview | brighter steel   | `#86B0D6` → `#10283F` |
| nightly | dark graphite    | `#4A525C` → `#0A0D11` |
| dev     | amber (a "not the real build" warning) | `#E0A75E` → `#40230C` |

Not rebranded on purpose: `assets/icons/` (403 functional editor glyphs) and the
`*_stamp.svg` / `zed_x_copilot.svg` assets (tied to Zed's paid backend, which the fork disables).

## Trademark

Zedium is an independent, unofficial fork and is **not affiliated with, endorsed by, or
sponsored by Zed Industries, Inc.** "Zed" is a trademark of Zed Industries, Inc.
These marks are Zedium's own work and do not incorporate Zed's logo, wordmark, or brand
colour. Always pair public use with the non-affiliation notice above.
