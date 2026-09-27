#!/bin/bash

BASEDIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

export PATH=$PATH:${BASEDIR}/target/release/:${BASEDIR}/src/runtime_lib/target/release/

export VERTEX_RUNTIME_PATH=${BASEDIR}/src/runtime_lib/target/release/libvm_runtime.a
