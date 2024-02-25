#!/bin/bash
# I have adhered to the Honor Code in this Assignment. Ankit Barana

for complex_num in "-0.4 + 0.65i" "-0.45 + 0.6i" "-0.45 + 0.55i" "-0.45 + 0.575i" "-0.5 + 0.575i"; do
    for size in "200" "400" "800"; do
        cargo run -- --constant="$complex_num" --size="$size" "Julia Set ${complex_num} ${size}x${size}.png"
    done
done