#!/bin/sh

RULE='{"tbl":[["Zero","Zero","Zero","Three","One","One","One","Three","Zero"],["Zero","Two","Zero","One","Three","Two","Three","Two","Two"],["Two","Two","Three","Two","Two","Zero","Two","One","One"],["Three","Three","Zero","Two","Two","Three","Two","One","Two"],["One","One","One","One","Zero","Two","Three","Zero","One"],["One","Three","Zero","Three","One","Zero","Two","One","Three"],["Zero","Zero","One","Two","Three","Two","Three","Zero","Two"],["Zero","One","Two","Three","Two","Zero","One","One","Three"],["Three","Two","Three","Zero","Three","Three","Three","Zero","Zero"]]}';
rm celaut.png
cargo run --bin generate -- $RULE
convert celaut.png -scale 500 celaut.png
feh celaut.png
