#!/bin/sh

RULE='';
rm celaut.png
cargo run --bin generate -- $RULE
convert celaut.png -scale 500 celaut.png
feh celaut.png
