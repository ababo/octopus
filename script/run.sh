#!/usr/bin/env bash

assert_defined () {
    if [ -z ${!1} ]; then
        echo "error: no $1 defined"
        exit 1
    fi
}

assert_defined PROJECT_PATH
assert_defined TARGET_ARCH
assert_defined TARGET_CPU
assert_defined TARGET_PLATFORM

kernel_dir=$PROJECT_PATH/target/$TARGET_ARCH-unknown-none/target/release
qemu-system-$TARGET_ARCH \
    -cpu $TARGET_CPU \
    -kernel $kernel_dir/octopus \
    -machine $TARGET_PLATFORM \
    -nographic
