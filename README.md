<!--
Copyright © The Daybrite Project
SPDX-License-Identifier: MPL-2.0
-->

# day-piece-crash-reporter

Capture crashes and let people review, send, or discard pending reports.

[Day](https://daybrite.dev/docs/) is a Rust framework for building apps with native desktop and
mobile controls, plus HTML controls on the web.

Initialize capture before app startup and configure a submission transport. The default `ui`
feature supplies a consent interface; capture does not upload reports automatically.

## Use

Add `day-piece-crash-reporter = "0.*"` to your app’s `[dependencies]`.
Follow the setup guide for platform requirements and initialization.

## Documentation

- [Setup and usage guide](https://daybrite.dev/docs/reference/break/)
- [Rust API](https://docs.rs/day-piece-crash-reporter/0.1.0/day_piece_crash_reporter/)
- [Crates.io](https://crates.io/crates/day-piece-crash-reporter)

[Source](https://github.com/daybrite/day-piece-crash-reporter) · [MPL-2.0 license](https://github.com/daybrite/day-piece-crash-reporter/blob/main/LICENSE)
