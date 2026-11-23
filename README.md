# GROVER

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

To check formatting, lint the server and run its tests in the running container, use

```sh
docker compose -f docker-compose.dev.yml exec web cargo fmt --all -- --check
docker compose -f docker-compose.dev.yml exec web cargo clippy -p igsn-web --features ssr -- -D warnings
docker compose -f docker-compose.dev.yml exec web cargo test -p igsn-web --features ssr
```

Press Ctrl+C to leave watch mode, then stop and remove the development container with

```sh
docker compose -f docker-compose.dev.yml down
```

## License

GROVER is licensed under the [GNU General Public License, version 3](LICENSE).
