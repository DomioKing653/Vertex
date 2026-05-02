#!/bin/bash
cargo clean
cargo update
cargo build --release
cd src/codegen
cargo build --lib --release
echo "vertexC and vertex are at ./target/release and vm is at src/codegen/target/release/libvm_runtime.a"
