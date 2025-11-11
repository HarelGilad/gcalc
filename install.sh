#!/usr/bin/env bash
set -e

echo "Building in release mode..."
cargo build --release

echo "Copying binary to ~/.local/bin/"
mkdir -p ~/.local/bin
cp target/release/gcalc  ~/.local/bin/
mkdir -p ~/.local/share/gcalc

echo "Installed ~/.local/bin/gcalc"
