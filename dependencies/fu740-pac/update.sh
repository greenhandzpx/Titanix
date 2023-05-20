#!/usr/bin/env bash
set -x
set -e

rm -rf src
mkdir src
svd patch fu740.yaml
svd2rust --target riscv -i fu740.svd.patched
form -i lib.rs -o src
rm lib.rs
cargo fmt
