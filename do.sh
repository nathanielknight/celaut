#!/bin/sh

RULE='{"tbl":[["Zero","Five","Zero"],["Zero","One","Zero"],["Zero","Zero","Zero"]]}';
rm celaut.png
cargo run -- $RULE
convert celaut.png -scale 500 celaut.png
feh celaut.png
