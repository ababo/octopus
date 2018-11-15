# Octopus Operating System


## How to build

### Prerequisites

1. Install [Rust nightly](https://doc.rust-lang.org/1.5.0/book/nightly-rust.html).
2. Run `cargo install cargo-xbuild`.
3. Set `ARCH`-environment variable for target architecture (can be `x86_64` or `aarch64`).
4. Follow the following steps depending on your host OS

### On Linux

1. Run `cargo xbuild --target $ARCH-octopus.json`

### On Mac OS

1. Install GNU binary utilities:
    - Download the latest [source code](http://ftp.gnu.org/gnu/binutils/).
    - Run `./configure --target $ARCH-elf`.
    - Run `make && sudo make install`.

2. Run the following commands:
    ```sh
    export RUSTFLAGS="-Clinker=/usr/local/bin/$ARCH-elf-ld -Clink-arg=--script=link.$ARCH.lds -Clink-arg=-zmax-page-size=1"
    cargo xbuild --target $ARCH-octopus.json --release
    ```


## How to test

The build produces multiboot-compatible kernel image which can be booted via GRUB. You can also use QEMU emulator to test it:

```sh
qemu-system-$ARCH -nographic -kernel target/$ARCH-octopus/release/octopus
```
