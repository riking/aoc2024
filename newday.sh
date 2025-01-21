#!/bin/sh

set -euo pipefail

git diff --cached --exit-code || (echo Dirty git staging!; exit 1)

echo Creating day $1

folder="day$1"
cargo init --bin $folder
cp template/src/main.rs $folder/src/main.rs
cd $folder
cargo add anyhow
git add .
git commit -m "Initial create $folder"

