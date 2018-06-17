#! /usr/bin/env bash
set -e

echo ""
echo "###################"
echo Bootstrapping quine-i
echo "###################"
cargo run -- i > i.rs

rustc i.rs
echo ""
echo "###################"
echo Running quine-i
echo "###################"
./i | tee o.rs

rustc o.rs
echo ""
echo "###################"
echo Running quine-o
echo "###################"
./o
