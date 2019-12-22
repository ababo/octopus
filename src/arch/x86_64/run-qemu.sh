#!/usr/bin/env sh

PROJECT_DIR="$(dirname "$0")/../../.."

qemu-system-x86_64 -kernel $PROJECT_DIR/target/x86_64-elf/release/octopus \
    -nographic
