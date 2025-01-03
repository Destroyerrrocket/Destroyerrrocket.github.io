#!/bin/bash

wasm-pack build --target web --release --no-pack --no-typescript --out-dir ../website/assets/wasm/triangle-demo triangle-demo
