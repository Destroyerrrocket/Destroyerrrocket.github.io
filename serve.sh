#!/bin/bash
# dx build --platform web -p website && cargo run -F "generate_htmls" ./target/dx/personal-webpage/debug/web/public &&
./generatesecondprojects.sh
dx serve --platform web -p website
