#!/bin/sh
rm -fr docs && rm -fr ./target/dx/personal-webpage/release/web/public && dx bundle --platform web --release --ssg && cargo run -F "generate_htmls" ./target/dx/personal-webpage/release/web/public
mv ./target/dx/personal-webpage/release/web/public/wasm/personal-webpage_bg.wasm ./target/dx/personal-webpage/release/web/public/wasm/personal-webpage_bg.bak.wasm
wasm-opt -Oz --strip-dwarf -o ./target/dx/personal-webpage/release/web/public/wasm/personal-webpage_bg.wasm ./target/dx/personal-webpage/release/web/public/wasm/personal-webpage_bg.bak.wasm
rm ./target/dx/personal-webpage/release/web/public/wasm/personal-webpage_bg.bak.wasm
mkdir -p docs
cp -r ./target/dx/personal-webpage/release/web/public/* docs
cp docs/assets/favicon.ico docs/favicon.ico
mv docs/assets/CNAME docs/CNAME
mv docs/assets/robots.txt docs/robots.txt
mv docs/assets/sitemap.txt docs/sitemap.txt