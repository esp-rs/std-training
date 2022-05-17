#!/bin/sh

set -ef

WORK_DIR=$HOME/workspace

echo "Compiling all exercises & library crates"
for file in $(find ${WORK_DIR} -name "Cargo.toml")
do
    cd $(dirname $file)
    echo "Checking $(pwd)"
    $HOME/.cargo/bin/cargo clean
    $HOME/.cargo/bin/cargo build
done
