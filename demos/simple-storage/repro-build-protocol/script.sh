#!/bin/bash

mkdir ./build
touch ./build/query.wasm

# TODO: Uncomment when packages are published
# npm i
# npm i -g assemblyscript@0.17.13

# asc  ./src/query/w3/entry.ts --path ./node_modules --outFile ./build/query.wasm --optimize --debug --importMemory --runtime stub
# asc  ./src/mutation/w3/entry.ts --path ./node_modules --outFile ./build/mutation.wasm --optimize --debug --importMemory --runtime stub