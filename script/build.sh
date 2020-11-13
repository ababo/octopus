#!/usr/bin/env bash

assert_defined () {
    if [ -z ${!1} ]; then
        echo "error: no $1 defined"
        exit 1
    fi
}

assert_defined PROJECT_PATH
assert_defined TARGET_ARCH
assert_defined TARGET_PLATFORM

target=$PROJECT_PATH/src/arch/$TARGET_ARCH/$TARGET_ARCH-unknown-none.json
cat $target | envsubst > /tmp/target.json

cd $PROJECT_PATH
cargo xbuild \
    --target /tmp/target.json \
    --target-dir target/$TARGET_ARCH-unknown-none \
    --release
