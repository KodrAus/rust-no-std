#!/bin/bash

set -o errexit -o nounset

rustup target add thumbv6m-none-eabi

cargo build --target=thumbv6m-none-eabi
