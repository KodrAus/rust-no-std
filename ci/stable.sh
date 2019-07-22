#!/bin/bash

set -o errexit -o nounset

cargo build
cargo build --features std

cargo test
cargo test --features std
