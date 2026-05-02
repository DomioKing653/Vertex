#!/bin/bash
cargo build
if [ $? -eq 0 ]; then
    ./target/debug/vertexC "$@"
fi
