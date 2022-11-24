#!/bin/bash
echo "Building for x86_64 linux"
cargo build --release -q --target x86_64-unknown-linux-gnu
echo "Building for x86_64 windows"
cargo build --release -q --target x86_64-pc-windows-gnu
echo "Building for i686 windows"
cargo build --release -q --target i686-pc-windows-gnu
echo "Done building releases!"