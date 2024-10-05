#!/bin/sh
rm -fr docs && dx build --release && cargo run
mv ./docs/assets/dioxus/personal-webpage_bg.wasm ./docs/assets/dioxus/personal-webpage_bg.bak.wasm
wasm-opt -Oz --strip-dwarf -o ./docs/assets/dioxus/personal-webpage_bg.wasm ./docs/assets/dioxus/personal-webpage_bg.bak.wasm
rm ./docs/assets/dioxus/personal-webpage_bg.bak.wasm