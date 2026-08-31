#!/bin/sh
# Installer for fish shell.
set -e

cmake -B build -DCMAKE_INSTALL_PREFIX="$HOME/.local"
cmake --build build
cmake --install build
