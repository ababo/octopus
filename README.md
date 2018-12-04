# Octopus Operating System


## How to build

### Prerequisites

1. Install [Rust nightly](https://doc.rust-lang.org/1.5.0/book/nightly-rust.html).
2. Run `cargo install cargo-xbuild`.
3. Set `TARGET_ARCH`-environment variable for target architecture (can be
`x86_64` or `aarch64`).
4. Cd to the project directory.
5. Follow the following steps depending on your host OS.

### On Linux

1. Run `cargo xbuild --target $TARGET_ARCH-octopus.json`.

### On Mac OS

1. Install GNU binary utilities:
    - Download the latest [source code](http://ftp.gnu.org/gnu/binutils/).
    - Run `./configure --target $TARGET_ARCH-elf`.
    - Run `make && sudo make install`.

2. Uncomment the target-specific options in `.cargo/config`.

2. Run `cargo xbuild --target $TARGET_ARCH-octopus.json`.


## How to test

The build produces multiboot-compatible kernel image which can be booted via GRUB. You can also use QEMU emulator to test it:

```sh
qemu-system-$ARCH -nographic -kernel target/$TARGET_ARCH-octopus/release/octopus
```

## Blog

See my development [blog](https://octopus-os.blogspot.com/).
