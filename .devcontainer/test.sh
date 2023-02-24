#!/bin/sh

set -ef

WORK_DIR=/workspace/$1

echo "Compiling $1"

cd /workspace/$1
$HOME/.cargo/bin/cargo clean
$HOME/.cargo/bin/cargo build
