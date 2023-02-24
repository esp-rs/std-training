#!/bin/sh

set -ef

WORK_DIR=/workspace/$1

echo "Compiling $1"

cd /workspace/$1
cargo clean
cargo build
