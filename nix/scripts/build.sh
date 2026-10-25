#!/usr/bin/env bash
# Build image

cd $(git rev-parse --show-toplevel 2>/dev/null)

cargo build --release
echo "Output:"
ls -lh target/riscv32imc-unknown-none-elf/release/xteink-x4
