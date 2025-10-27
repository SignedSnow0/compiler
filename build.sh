#!/bin/bash

if [ $# -ne 1 ]; then
    echo "Usage: build.sh <file>"

    exit 1
fi

cargo run $1 && clang "$1.ll" -o "$1.out"
