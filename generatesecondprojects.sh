#!/bin/bash

pushd triangle-demo
wasm-pack build --target web --release --out-dir ../website/assets/wasm/triangle-demo
popd