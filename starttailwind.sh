#!/bin/bash
pushd ./website
npx tailwindcss -i ./src/tailwind/input.css -o ./assets/css/tailwind.css --watch --minify
popd