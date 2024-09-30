#!/bin/sh
rm -fr docs && dx build --release && cp docs/index.html docs/404.html