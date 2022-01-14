#!/usr/bin/env bash

# Initializing build environment


rustup update nightly
rustup update stable
rustup default nightly

rustup target add wasm32-unknown-unknown --toolchain nightly
