# Zedium Changelog

This file tracks what Zedium carries on top of [Zed](https://github.com/zed-industries/zed). Upstream `main` is merged regularly, so upstream features are not listed here — only Zedium-specific ports, additions, and divergences.

Credit matters: Zedium exists because of work the community already did. Ported entries name the original pull request and its authors, and their commits keep original authorship in our git history. If an upstream PR we ported later merges into Zed, we drop our port and absorb upstream's version.

## Unreleased

### Ported from upstream PRs

- Added `editor::PastePrimarySelection` and `terminal::PastePrimarySelection`; on Linux, `shift-insert` now pastes from the X11/Wayland PRIMARY selection as is conventional, instead of the regular clipboard ([#15](https://github.com/zedium-editor/zedium/pull/15)). Ported from [zed-industries/zed#53447](https://github.com/zed-industries/zed/pull/53447) by [@zaugust4891](https://github.com/zaugust4891), co-authored by Jason Garber. Addresses long-standing requests in zed-industries/zed issues [#44695](https://github.com/zed-industries/zed/issues/44695), [#36618](https://github.com/zed-industries/zed/issues/36618), and [#45482](https://github.com/zed-industries/zed/issues/45482).

### Changed from upstream defaults

- Telemetry is disabled by default.
- Automatic updates are disabled; Zedium releases are delivered through this repository's releases.
