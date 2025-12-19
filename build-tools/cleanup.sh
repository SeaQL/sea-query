#!/bin/bash
set -x

for dir in ./examples/*; do
    cd "$dir";
    cargo clean;
    cd ..;
done