#!/bin/sh

set -euo pipefail

echo Creating day $1

folder="day$1"
cargo init --bin $folder
cp template/src/main.rs $folder/src/main.rs
cd $folder
cargo add anyhow


