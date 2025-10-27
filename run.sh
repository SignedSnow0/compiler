#!/bin/bash

if [ $# -ne 1 ]; then
    echo "Usage: run.sh <file>"

    exit 1
fi

./build.sh "$1" && "./$1.out" 
echo $?
