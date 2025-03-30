#!/bin/sh
./generatesecondprojects.sh
rm -fr docs && rm -fr ./target/dx/website/release/web/public && dx bundle --platform web --release -p website && cargo run -p website -F "generate_htmls" ./target/dx/website/release/web/public
mv ./target/dx/website/release/web/public/wasm/website_bg.wasm ./target/dx/website/release/web/public/wasm/website_bg.bak.wasm
wasm-opt -Oz --strip-dwarf -o ./target/dx/website/release/web/public/wasm/website_bg.wasm ./target/dx/website/release/web/public/wasm/website_bg.bak.wasm
rm ./target/dx/website/release/web/public/wasm/website_bg.bak.wasm
mkdir -p docs
cp -r ./target/dx/website/release/web/public/* docs
cp docs/assets/favicon.ico docs/favicon.ico
mv docs/assets/CNAME docs/CNAME
mv docs/assets/robots.txt docs/robots.txt
mv docs/assets/sitemap.txt docs/sitemap.txt
rm -fr ./static