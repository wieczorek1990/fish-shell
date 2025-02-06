#!/usr/bin/env bash
# Installs fish dependencies with brew.

brew install --formula\
 rust cmake gcc pcre2 gettext\
 python tmux sphinx-doc
pip install pexpect
