#! /usr/bin/env bash
set -e

echo ""
echo "###################"
echo Bootstrapping quine-o
echo "###################"
cargo run -- o > o.rs

rustc o.rs
echo ""
echo "###################"
echo Running quine-o
echo "###################"
./o | tee i.rs

rustc i.rs
echo ""
echo "###################"
echo Running quine-i
echo "###################"
./i
