# day-piece-break

Crash capture and reporting UI for [Day](https://daybrite.dev) apps. Captures Rust panics,
Unix signals, and Android uncaught Java exceptions; reconciles reports on the next launch;
and provides a localized consent banner with **View**, **Send**, and **Discard** actions.
Nothing is uploaded automatically.

```toml
[dependencies]
day-piece-break = { git = "https://github.com/daybrite/day-piece-break" }
```

Initialize before `day::launch`:

```rust
let _ = day_piece_break::Config::new()
    .reporter(day_piece_break::EmailReporter::new("support@example.com"))
    .init();
```

Embed `day_piece_break::consent_banner()` in your root's layout. It appears when a report is
pending and offers the full report, destination disclosure, submission status, and dismissal.
The UI is ordinary Day composition, so it works with the app's selected backend without
additional backend feature flags. Its embedded translations cover English, French, Arabic,
and Simplified Chinese.

For a custom interface, disable the default `ui` feature. Crash capture, report storage, and
the `Reporter`, `RestReporter`, `GithubIssueReporter`, and `EmailReporter` APIs remain available.

[Full reference](docs/break.md) · [Showcase integration](https://github.com/daybrite/Day-Showcase/blob/main/src/pages/crash.rs)

## Compatibility and migration

Extracted from `day/crates/day-break` under MPL-2.0. Rename Rust references from `day_break::`
to `day_piece_break::`, or use Cargo's `package = "day-piece-break"` dependency alias to keep
an existing import name. The API otherwise stays the same.

The `day-break` report directory, schema-1 JSON, `DAY_BREAK_*` environment variables,
Android Java/JNI symbols, and `dbreak-*` element IDs are intentionally unchanged, so existing
reports and automation continue working. Reports identify the linked runtime using
`day_core::VERSION`, rather than this independently versioned package's version.

Dependencies use Day's bare canonical git URL, letting the app's lockfile select one Day
revision for the whole graph. This initial release targets Day 0.4 with the `day_core::VERSION`
API. No library lockfile is committed; CI checks compatibility against Day's current main.
On wasm, initialization and capture are no-ops. Native signal capture is Unix-only; Windows
captures Rust panics. See the reference for other platform limits.

## Development

For a local Day checkout:

```sh
day patch --local ../day
cargo test
cargo test --no-default-features
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

The subprocess tests deliberately crash child processes and verify report recovery; the
mock UI test drives the consent banner. CI runs both feature configurations on macOS, Linux,
and Windows, and compile-checks iOS, Android, HarmonyOS, and web. Changes to this package can
be tested in Day-Showcase using `day patch --local ../day-piece-break`.

## Publishing the extraction

The new repository and Day-Showcase's migrated dependency must be available before removing
`day-break` from the public Day workspace used by Showcase CI. Land the additive
`day_core::VERSION` API first, publish this repository, migrate Showcase, then land the workspace
removal. These are separate publication steps even though the local checkouts can be tested
together with path patches.
