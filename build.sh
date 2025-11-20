#!/bin/bash

if [[ $* == *--no-opt* ]]; then
    cargo run main.d && clang main.d.ll -o main
else
    cargo run main.d && opt -S -p mem2reg main.d.ll -o main.d.ll && clang main.d.ll -o main
fi

