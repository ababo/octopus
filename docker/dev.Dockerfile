FROM debian:10.6-slim

ARG TOOLCHAIN=nightly-2020-11-10
ARG PROJECT_PATH=/octopus

ENV PATH /root/.cargo/bin:$PATH
ENV PATH $PROJECT_PATH/script:$PATH

RUN apt-get update \
    && apt-get install -y curl gettext-base build-essential \
        binutils-x86-64-linux-gnu binutils-aarch64-linux-gnu \
        qemu-system-x86 qemu-system-arm \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
        sh -s -- -y --default-toolchain $TOOLCHAIN \
    && rustup component add rust-src \
    && cargo install cargo-xbuild
