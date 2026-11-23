# GROVER

[![Rust](https://img.shields.io/badge/Rust-1.100.0-382e2a?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-0.8-445469)](https://docs.rs/axum/)
[![Tokio](https://img.shields.io/badge/Tokio-1-364552)](https://tokio.rs/)
[![Leptos](https://img.shields.io/badge/Leptos-0.8-a44228)](https://leptos.dev/)
[![Docker Compose](https://img.shields.io/badge/Docker-Compose-1769a5?logo=docker&logoColor=white)](https://docs.docker.com/compose/)

[![Rust tests](https://github.com/McNamara84/grover/actions/workflows/rust-tests.yml/badge.svg?branch=main)](https://github.com/McNamara84/grover/actions/workflows/rust-tests.yml)
[![Playwright browser tests](https://github.com/McNamara84/grover/actions/workflows/browser-tests.yml/badge.svg?branch=main)](https://github.com/McNamara84/grover/actions/workflows/browser-tests.yml)
[![Native Rust line coverage](coverage/rust-lines.svg)](#coverage)
[![Native Rust function coverage](coverage/rust-functions.svg)](#coverage)

**Generator of Researcher-friendly, Ontology-aligned, Validated Excel templates for Research samples**

GROVER is being developed for researchers preparing physical samples for IGSN registration. An International Generic Sample Number (IGSN) gives a sample a persistent identifier. [DataCite’s guide to IGSN IDs](https://support.datacite.org/docs/igsn-ids)

Researchers will enter shared project metadata once in the browser and download a prefilled Excel workbook (`.xlsx`). They will then enter individual samples row by row. The planned templates follow the IGSN Core Schema 2.0. Discipline-specific profiles, starting with the Geo profile, will extend the core fields and validation rules.

The first development milestone is a working home page. The Rust workspace currently contains `crates/igsn-web`. Its Axum server runs on Tokio, renders the page with Leptos and serves the WebAssembly bundle that hydrates it in the browser. The application is stateless.

## Local development

Use Docker Desktop with Linux containers, or Docker Engine, with Docker Compose 2.22 or later. The development image includes the Rust toolchain and `cargo-leptos`.

From the repository root, run

```sh
docker compose -f docker-compose.dev.yml up --build --watch
```

The first start downloads the toolchain and compiles both the server and browser bundle. Once the logs show `Serving GROVER`, open [the local home page](http://localhost:3000/).

Keep the command running while editing. Compose syncs changes under `crates/igsn-web/src` and `crates/igsn-web/style`. Leptos rebuilds the affected code and reloads the browser using port 3001. Both ports are bound to the loopback interface. Changes to the Cargo manifests, lockfile or development Dockerfile rebuild the image. A Docker volume caches compiled artifacts between starts.

To follow the server logs from another terminal, run

```sh
docker compose -f docker-compose.dev.yml logs -f web
```

Press Ctrl+C to leave watch mode, then stop and remove the development container with

```sh
docker compose -f docker-compose.dev.yml down
```

## Testing

The Rust tests send HTTP requests through the Axum router. They check the initial HTML, the response to a missing browser asset and the status of an unsupported request method. CI runs Clippy for both the native server (`ssr`) and the WebAssembly client (`hydrate`). To run the same checks and tests in the development container, use

```sh
docker compose -f docker-compose.dev.yml exec web cargo fmt --all -- --check
docker compose -f docker-compose.dev.yml exec web cargo clippy --locked --workspace --features ssr --all-targets -- -D warnings
docker compose -f docker-compose.dev.yml exec web cargo clippy --locked -p igsn-web --lib --target wasm32-unknown-unknown --no-default-features --features hydrate -- -D warnings
docker compose -f docker-compose.dev.yml exec web cargo test --locked --workspace --features ssr
```

For Clippy diagnostics on save, editors using rust-analyzer can set `rust-analyzer.check.command` to `clippy`. [rust-analyzer configuration](https://rust-analyzer.github.io/book/configuration.html#rust-analyzer.check.command)

Before launching browsers, CI checks the Playwright configuration and test files with TypeScript in strict mode and with type-aware Oxlint. For example, an unawaited Playwright assertion fails the `typescript/no-floating-promises` rule. To run both checks locally, use

```sh
docker compose -f docker-compose.dev.yml run --rm --no-deps --build --entrypoint npm e2e run check
```

Playwright checks the exact notice with JavaScript disabled. With JavaScript enabled, it loads the WebAssembly bundle and exercises the hydrated router using Back and Forward. The following command builds the test image, starts the web service if needed and waits for it to serve HTTP before opening a browser.

```sh
docker compose -f docker-compose.dev.yml run --rm --build e2e --project=chromium
```

Omit `--project=chromium` to run Chromium, Firefox and WebKit. Pull requests run Chromium. Pushes to `main`, version tags (`v*`) and manual workflow runs exercise all three engines. Playwright saves failure traces in `target/playwright/`. CI makes them available as the `playwright-traces` artifact.

### Coverage

To run the native Rust tests with LLVM instrumentation and refresh the two coverage badges, use

```sh
docker compose -f docker-compose.dev.yml run --rm --build coverage
```

The percentages measure application lines and functions compiled with the `ssr` feature. The coverage report excludes test source files. The Playwright E2E badge reports the browser results. Open `coverage/report/html/index.html` to inspect individual lines or use `coverage/report/summary.json` for the totals. Commit the refreshed SVGs with changes that affect the measurements. The Rust workflow checks them against its own run and uploads the `rust-coverage` report.

### Tests for the next milestones

| Milestone | Acceptance checks |
| --- | --- |
| Profile registry | Use `proptest` to generate combinations of optional fields and sample hierarchy levels. Check field identity and rejection of conflicting paths. Adopt `cargo-nextest` when multiple Rust crates have test suites. |
| First XLSX export | Read generated workbooks with `calamine`. Use `insta` to review snapshots of sheet names, column order and sample values. Check dropdown ranges, defined names and sheet protection in the workbook XML. |
| First project form and release | Add Playwright cases for correcting an invalid input and downloading a configured workbook. Before release, open a reference workbook in Excel and LibreOffice, choose a dropdown value, enter a sample, save and reopen it. |

## License

GROVER is licensed under the [GNU General Public License, version 3](LICENSE).
