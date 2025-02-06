#!/usr/bin/env bash
# Install using cargo.
# This script installs into ~/.cargo/bin/.

BASEDIR="$(dirname "$0")"

cd "${BASEDIR}"/../ || exit
cargo install --path .
cd - || exit
