#!/usr/bin/env sh
if [[ ! -d "hello-world" ]]; then
	cargo new --bin "hello-world" --vcs none
fi
cd "hello-world"
cargo run
cargo clean

