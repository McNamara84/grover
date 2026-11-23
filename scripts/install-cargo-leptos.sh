#!/bin/sh
set -eu

version=${1:-0.3.11}
host=$(rustc -vV | sed -n 's/^host: //p')
archive="cargo-leptos-${host}.tar.gz"
release="https://github.com/leptos-rs/cargo-leptos/releases/download/v${version}"

cd /tmp
curl -fsSLO --retry 3 "${release}/${archive}"
curl -fsSLO --retry 3 "${release}/${archive}.sha256"
sha256sum --check "${archive}.sha256"
tar -xzf "${archive}" -C "${CARGO_HOME}/bin" --strip-components=1 "cargo-leptos-${host}/cargo-leptos"
rm "${archive}" "${archive}.sha256"
