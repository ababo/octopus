#!/usr/bin/env sh

PROJECT_DIR="$(dirname "$0")/../../.."

qemu-system-aarch64 -kernel $PROJECT_DIR/target/aarch64-elf/release/octopus \
    -machine virt -cpu cortex-a53 -nographic
