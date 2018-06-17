#! /usr/bin/env bash
./bootstrap_i.sh

pushd render
cargo run -- -i ../o.rs -o ../i.pdf
cargo run -- -i ../i.rs -o ../o.pdf
popd
